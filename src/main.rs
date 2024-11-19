// Most of this comes from:
// https://github.com/jabedude/ja3-rs/blob/master/src/lib.rs

use std::net::TcpListener;

use lazy_static::*;
use md5::{self, Digest};
use tls_parser::parse_tls_plaintext;
use tls_parser::{parse_tls_extensions, TlsExtension, TlsExtensionType};
use tls_parser::{TlsMessage, TlsMessageHandshake, TlsRecordType};

lazy_static! {
    static ref GREASE: Vec<u16> = vec![
        0x0a0a, 0x1a1a, 0x2a2a, 0x3a3a, 0x4a4a, 0x5a5a, 0x6a6a, 0x7a7a, 0x8a8a, 0x9a9a, 0xaaaa,
        0xbaba, 0xcaca, 0xdada, 0xeaea, 0xfafa
    ];
}

#[derive(Debug)]
pub struct Ja3Hash {
    /// The string consisting of the SSLVersion,Cipher,SSLExtension,EllipticCurve,EllipticCurvePointFormat
    /// See the original [JA3 specification](https://github.com/salesforce/ja3#how-it-works) for more info.
    pub ja3_str: String,
    /// The MD5 hash of `ja3_str`.
    pub hash: Digest,
}

fn process_extensions(extensions: &[u8]) -> Option<String> {
    let mut ja3_exts = String::new();
    let mut supported_groups = String::new();
    let mut ec_points = String::new();
    let (_, exts) = parse_tls_extensions(extensions).unwrap();
    for extension in exts {
        let ext_val = u16::from(TlsExtensionType::from(&extension));
        if GREASE.contains(&ext_val) {
            continue;
        }
        println!("Ext: {:?}", ext_val);
        ja3_exts.push_str(&format!("{}-", ext_val));
        match extension {
            TlsExtension::EllipticCurves(curves) => {
                for curve in curves {
                    if !GREASE.contains(&curve.0) {
                        println!("curve: {}", curve.0);
                        supported_groups.push_str(&format!("{}-", curve.0));
                    }
                }
            }
            TlsExtension::EcPointFormats(points) => {
                println!("Points: {:x?}", points);
                for point in points {
                    ec_points.push_str(&format!("{}-", point));
                }
            }
            _ => {}
        }
    }
    ja3_exts.pop();
    supported_groups.pop();
    ec_points.pop();
    println!("Supported groups: {}", supported_groups);
    println!("EC Points: {}", ec_points);
    let ret = format!("{},{},{}", ja3_exts, supported_groups, ec_points);
    Some(ret)
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut buf: Vec<u8> = vec![0; 1024];
                let len = stream.peek(&mut buf).expect("peek failed");
                let bytes = &buf[0..len];
                let mut ja3_string = String::new();
                let res = parse_tls_plaintext(&bytes);
                match res {
                    Ok((rem, record)) => {
                        println!("Rem: {:?}, record: {:?}", rem, record);
                        println!("record type: {:?}", record.hdr.record_type);
                        if record.hdr.record_type != TlsRecordType::Handshake {
                            return ();
                        }
                        for rec in record.msg {
                            if let TlsMessage::Handshake(handshake) = rec {
                                if let TlsMessageHandshake::ClientHello(contents) = handshake {
                                    println!("handshake contents: {:?}", contents);
                                    println!(
                                        "handshake tls version: {:?}",
                                        u16::from(contents.version)
                                    );
                                    ja3_string
                                        .push_str(&format!("{},", u16::from(contents.version)));
                                    for cipher in contents.ciphers {
                                        println!("handshake cipher: {}", u16::from(cipher));
                                        if !GREASE.contains(&cipher) {
                                            ja3_string.push_str(&format!("{}-", u16::from(cipher)));
                                        }
                                    }
                                    ja3_string.pop();
                                    ja3_string.push(',');
                                    if let Some(extensions) = contents.ext {
                                        let ext = process_extensions(extensions).unwrap();
                                        ja3_string.push_str(&ext);
                                    }
                                }
                            }
                        }
                    }
                    _ => {
                        eprintln!("ERROR");
                        return ();
                    }
                }
                let hash = md5::compute(&ja3_string.as_bytes());
                println!("JA3: {}", ja3_string);
                println!("Hash: {:x}", hash);
            }
            Err(_e) => {}
        }
    }
}
