use std::error::Error as StdError;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::Arc;

use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::server::Acceptor;

fn main() -> Result<(), Box<dyn StdError>> {
    let certs = CertificateDer::pem_file_iter("cert.pem")
        .unwrap()
        .map(|cert| cert.unwrap())
        .collect();
    let private_key = PrivateKeyDer::from_pem_file("key.pem").unwrap();
    let config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, private_key)?;
    let arc_config = Arc::new(config);

    let listener = TcpListener::bind(format!("[::]:{}", 4443)).unwrap();
    //let (mut stream, _) = listener.accept()?;

    //let mut conn = rustls::ServerConnection::new(Arc::new(config))?;

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut acceptor = Acceptor::default();
        let accepted = loop {
            acceptor.read_tls(&mut stream).unwrap();
            if let Some(accepted) = acceptor.accept().unwrap() {
                let client_hello = accepted.client_hello();
                println!("Cipher suites: {:?}", client_hello.cipher_suites());
                break accepted;
            }
        };

        // For some user-defined choose_server_config:
        //let config = choose_server_config(accepted.client_hello());
        let mut conn = accepted.into_connection(arc_config.clone()).unwrap();

        // Proceed with handling the ServerConnection.
        conn.complete_io(&mut stream)?;

        let response = "HTTP/1.1 200 OK\r\nContent-Length: 20\r\n\r\nHello from server";
        conn.writer().write_all(response.as_bytes())?;
        conn.complete_io(&mut stream)?;

        let mut buf = [0; 64];
        let len = conn.reader().read(&mut buf)?;
        println!("Received message from client: {:?}", &buf[..len]);
    }
    Ok(())
}

