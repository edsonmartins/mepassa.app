#!/bin/bash
# Health check for coturn TURN server

# Check if TURN ports are listening
if ! nc -zv localhost 3478 2>&1 | grep -q succeeded; then
    echo "ERROR: TURN port 3478 (UDP/TCP) not listening"
    exit 1
fi

if ! nc -zv localhost 5349 2>&1 | grep -q succeeded; then
    echo "ERROR: TURNS port 5349 (TLS) not listening"
    exit 1
fi

echo "OK: coturn is healthy"
exit 0
