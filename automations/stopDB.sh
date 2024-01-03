#!/bin/bash

# This script stops the following:
# 1. Stops the dockerized version of SurrealDB

# Load the process ID saved during the start script
db_process_pid=$(<../automations/.db_process_pid.txt)

if [ -z "$db_process_pid" ]; then
    echo "Error: Could not retrieve SurrealDB process ID. Is the database running?"
    exit 1
fi

# Stop the SurrealDB Docker Compose process
echo "Stopping SurrealDB Docker Compose process (PID: $db_process_pid)..."
kill $db_process_pid

# Clean up: Remove the file storing the process ID
rm ../automations/.db_process_pid.txt

# Set up environment variables
source ../.env

# Stop the dockerized version of SurrealDB
cd ../database
ls
docker-compose down