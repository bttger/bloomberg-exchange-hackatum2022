#!/bin/sh

server='ws://127.0.0.1:3000/api'

echo '{"Add":{"user":"jalil","stock":"IBM","side":"sell","price":10,"quantity":100}}' | websocat "$server"
