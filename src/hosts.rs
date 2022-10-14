use std::fs::{read, write};
use std::path::Path;

const HOSTS_PATH: &str = "C:/Windows/System32/drivers/etc/hosts";

pub fn has_host_redirect() -> bool {
    let file_path = Path::new(HOSTS_PATH);
    if !file_path.exists() {
        return false;
    }
    let contents = read(file_path).unwrap();
    let contents = String::from_utf8_lossy(&contents)
        .to_string();
    let mut lines = contents.lines();
    let line = lines.find(|line| (*line).contains("127.0.0.1 gosredirector.ea.com"));
    line.is_some()
}

pub fn add_hosts_redirect() {
    let file_path = Path::new(HOSTS_PATH);
    if !file_path.exists() {
        return;
    }
    let contents = read(file_path).unwrap();
    let contents = String::from_utf8_lossy(&contents)
        .to_string();
    let mut lines = contents.lines()
        .filter(|line| !(*line).contains("gosredirector.ea.com"))
        .collect::<Vec<&str>>();
    lines.push("127.0.0.1 gosredirector.ea.com");
    let out = lines.join("\r\n");
    write(file_path,out)
        .expect("Unable to write hosts file");
}

pub fn remove_hosts_redirect() {
    let file_path = Path::new(HOSTS_PATH);
    if !file_path.exists() {
        return;
    }
    let contents = read(file_path).unwrap();
    let contents = String::from_utf8_lossy(&contents)
        .to_string();
    let mut lines = contents.lines();
    let out = lines
        .filter(|line| !(*line).contains("gosredirector.ea.com"))
        .collect::<Vec<&str>>()
        .join("\r\n");

    write(file_path, out)
        .expect("Unable to write hosts file");
}