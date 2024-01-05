#!/bin/bash

# This script sets up the following:
# 1. Sets up the environment variables for the services
# 2. Runs the dockerized version of SurrealDB
# 3. Compiles the release version of migrations rust binary
# 4. Copies the sql folder from the migrations folder to migrations/target/release

# Set up environment variables
source ../.env

# Run the dockerized version of SurrealDB
cd ../database
rm -rf ../automations/logs/*
docker-compose up > ../automations/logs/db.log 2>&1 &

# Get the process ID of the background process
db_process_pid=$!

# Save the process ID to a file for later reference
echo $db_process_pid > ../automations/.db_process_pid.txt

# Wait for the database to start
sleep 1

# Compile the release version of migrations rust binary
cd ../migrations
cargo build --release

# Copy the sql folder from the migrations folder to migrations/target/release and before that delete the existing sql folder
rm -rf target/release/sql
cp -r sql target/release

# Run the release version of migrations rust binary
cd target/release
./mer-migrations