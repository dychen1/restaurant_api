#!/bin/bash

SCRIPT_ROOT="$(cd "$(dirname "$0")" && pwd)"
ENV_FILE=.env

cd "$SCRIPT_ROOT" && set -a && source "$ENV_FILE"
echo "Environment configured!"

echo "Starting up docker container!"

# Force run init.sql each run
docker rm "$DATABASE_CONTAINER" 2>/dev/null
docker-compose up --build --force-recreate --always-recreate-deps