#!/bin/bash

SCRIPT_ROOT="$(cd "$(dirname "$0")" && pwd)"
ENV_FILE=.env

cd "$SCRIPT_ROOT" && set -a && source "$ENV_FILE"
echo "Environment configured!"

echo "Starting up docker container!"

# Force run init.sql each run
docker rm "$DATABASE_CONTAINER" 2>/dev/null || true
docker-compose up --build --force-recreate --always-recreate-deps -d
cargo build --release

echo "Waiting for the database container to start..."
sleep 12 # Quick hack to wait for the database container to start, should be replaced with a proper health check

cargo run --release