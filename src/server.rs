use std::{io, thread};
use std::io::{Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, PoisonError, RwLock};
use blaze_pk::{group, OpaquePacket, packet, PacketError, Packets, TdfOptional};
use blaze_ssl::stream::SslStream;
use crate::shared::SharedState;


/// Starts the server
pub fn run_server(state: Arc<RwLock<SharedState>>) {
    // Begin listening for connections
    let listener = TcpListener::bind(("0.0.0.0", 42127))
        .expect("Failed to bind TCP listener");

    for stream in listener.incoming() {
        let state = state.clone();
        thread::spawn(move || {
            let stream = stream.expect("Failed to accept stream");
            let stream =
                &mut SslStream::new(stream).expect("Failed to complete handshake");
            let _ = handle_client(stream, state);
        });
    }
}

#[derive(Debug)]
pub enum ClientError {
    LockPoison,
    IOError(io::Error),
    PacketError(PacketError)
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

impl From<PacketError> for ClientError {
    fn from(err: PacketError) -> Self {
        ClientError::PacketError(err)
    }
}

packet! {
    struct RedirectPacket {
        ADDR: TdfOptional<AddressGroup>,
        SECU: bool,
        XDNS: bool,
    }
}

group! {
    struct AddressGroup {
        HOST: String,
        PORT: u16
    }
}

pub fn handle_client(
    stream: &mut SslStream<TcpStream>,
    state: Arc<RwLock<SharedState>>,
) -> Result<(), ClientError> {
    let packet = OpaquePacket::read(stream)?;

    println!("{packet:?}");

    let state = state.read()?;
    let content = RedirectPacket {
        ADDR: TdfOptional::default_some("VALUE", AddressGroup {
            HOST: state.host.clone(),
            PORT: state.port,
        }),
        SECU: false,
        XDNS: false
    };

    let response = Packets::response(&packet, content);
    response.write(stream)?;


    stream.flush()?;
    // stream.shutdown()?;
    Ok(())
}