#!/bin/bash
set -euf

SCRIPT_ROOT="$(cd "$(dirname "$0")"; pwd)"
ENV_FILE=.env

cd "$SCRIPT_ROOT" && set -a && source $ENV_FILE
echo "Environment configured!"

echo "Starting up docker container!"
docker-compose build
docker-compose up
