# check-tls

Create key and cert:
```bash
openssl genrsa -out key.pem 2048
openssl req -new -x509 -key key.pem -out cert.pem -days 365 -subj "/CN=localhost"
```

Run server:
```bash
cargo run
```

Connect with curl:
```bash
curl -k https://localhost:4443
```

Example output:
```
Cipher suites: [TLS13_CHACHA20_POLY1305_SHA256, TLS13_AES_256_GCM_SHA384, TLS13_AES_128_GCM_SHA256, TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256, TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256, CipherSuite(0xccaa), TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384, TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384, TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA384, TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA384, TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA, TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA, CipherSuite(0x9f), CipherSuite(0x6b), CipherSuite(0x39), CipherSuite(0xff85), CipherSuite(0xc4), CipherSuite(0x88), CipherSuite(0x81), CipherSuite(0x9d), CipherSuite(0x3d), CipherSuite(0x35), CipherSuite(0xc0), CipherSuite(0x84), TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256, TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256, TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA256, TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA256, TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA, TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA, CipherSuite(0x9e), CipherSuite(0x67), CipherSuite(0x33), CipherSuite(0xbe), CipherSuite(0x45), CipherSuite(0x9c), CipherSuite(0x3c), CipherSuite(0x2f), CipherSuite(0xba), CipherSuite(0x41), CipherSuite(0xc011), CipherSuite(0xc007), CipherSuite(0x5), CipherSuite(0x4), CipherSuite(0xc012), CipherSuite(0xc008), CipherSuite(0x16), CipherSuite(0xa), TLS_EMPTY_RENEGOTIATION_INFO_SCSV]
```

Docs on Cipher Suites from rustls: https://docs.rs/rustls/latest/rustls/enum.CipherSuite.html
