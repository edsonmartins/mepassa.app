#!/usr/bin/bash
# Health check for coturn TURN server.
# A imagem coturn/coturn não tem `nc`; usamos /dev/tcp do bash.
# TLS/DTLS está desligado em dev (no-tls/no-dtls) - só checamos a porta 3478.

if ! bash -c 'exec 3<>/dev/tcp/127.0.0.1/3478' 2>/dev/null; then
    echo "ERROR: TURN port 3478 (UDP/TCP) not listening"
    exit 1
fi

echo "OK: coturn is healthy"
exit 0
