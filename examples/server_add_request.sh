#!/bin/sh

server='http://localhost:3000/api/add'

curl -X POST -H 'Content-Type: application/json' -d '' "$server"
