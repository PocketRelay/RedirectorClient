use std::{io, thread};
use std::io::{Cursor, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, PoisonError, RwLock};
use blaze_pk::error::TdfError;
use blaze_pk::io::Writable;
use blaze_pk::packet::{DecodedPacket, Packet};
use blaze_pk::tdf::Tdf;
use blaze_schannel::{CertStore, SchannelCred, TlsStream};
use crate::shared::SharedState;

#[derive(Debug)]
pub enum ServerError {
    IO(io::Error),
}

/// Starts the server
pub fn run_server(state: Arc<RwLock<SharedState>>) -> Result<(), ServerError> {
    // Import redirector store
    let cert_store = {
        let store_bytes = include_bytes!("redirector.pfx");
        let store_password = "123456";

        CertStore::import_pkcs12(store_bytes, Some(store_password))
            .expect("Failed to import store")
    };

    // Load certificate from store
    let cert = cert_store
        .first()
        .expect("Store missing certificate");

    // Create schannel credentials
    let cred = SchannelCred::new(cert.clone())
        .expect("Failed to create schannel cred");

    // Begin listening for connections
    let listener = TcpListener::bind(("0.0.0.0", 42127))
        .expect("Failed to bind TCP listener");

    for stream in listener.incoming() {
        let cred = cred.clone();
        let state = state.clone();
        thread::spawn(move || {
            let stream = stream.expect("Failed to accept stream");
            let stream = &mut TlsStream::new(cred, stream)
                .expect("Failed to complete handshake");
            let _ = handle_client(stream, state);
        });
    }
    Ok(())
}

#[derive(Debug)]
pub enum ClientError {
    LockPoison,
    IOError(io::Error),
    PacketError
}

impl<Guard> From<PoisonError<Guard>> for ClientError {
    fn from(_: PoisonError<Guard>) -> Self {
        ClientError::LockPoison
    }
}

impl From<io::Error> for ClientError {
    fn from(err: io::Error) -> Self {
        ClientError::IOError(err)
    }
}

impl From<TdfError> for ClientError {
    fn from(_: TdfError) -> Self {
        ClientError::PacketError
    }
}

pub fn handle_client(
    stream: &mut TlsStream<TcpStream>,
    state: Arc<RwLock<SharedState>>,
) -> Result<(), ClientError> {
    let packet = DecodedPacket::read(stream)?;
    let content = {
        let state = state.read()?;
        let content = vec![
            Tdf::optional("ADDR", 0, Some(
                Tdf::group("VALUE", vec![
                    Tdf::str("HOST", &state.host),
                    Tdf::num("PORT", (*&state.port) as u64),
                ])
            )),
            Tdf::bool("SECU", false),
            Tdf::bool("XDNS", false),
        ];
        content
    };
    let response = Packet::response(&packet, content);
    let out_cursor = &mut Cursor::new(Vec::new());
    response.write(out_cursor)?;
    let stream = stream;
    let bytes = out_cursor.get_mut();
    stream.write_all(bytes)?;
    stream.shutdown()?;
    Ok(())
}