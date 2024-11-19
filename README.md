# tls-ja3

Mostly based on https://github.com/jabedude/ja3-rs
but using a tiny TcpListener instead of a socket or pcap

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
```bash
JA3: 771,4867-4866-4865-52393-52392-52394-49200-49196-49192-49188-49172-49162-159-107-57-65413-196-136-129-157-61-53-192-132-49199-49195-49191-49187-49171-49161-158-103-51-190-69-156-60-47-186-65-49169-49159-5-4-49170-49160-22-10-255,43-51-0-11-10-13-16,29-23-24-25,0

Hash: 375c6162a492dfbf2795909110ce8424
```
