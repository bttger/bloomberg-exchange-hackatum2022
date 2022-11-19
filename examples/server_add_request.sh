#!/bin/sh

server='ws://127.0.0.1:3000/'

echo '{"Add":{"user_id":"jalil","symbol":"IBM","type":"sell","price":10,"amount":100}}' | websocat "$server"
