#!/bin/bash

cd ..

# This script sets up the following:
# 1. Sets up the environment variables for the services

# 2. Runs the dockerized version of SurrealDB

# 3. Compiles the release version of migrations rust binary
# 4. Copies the sql folder from the migrations folder to migrations/target/release

# 5. Compiles the release version of the driver rust binary
# 6. Runs the release version of the driver rust binary

# Set up environment variables
source .env

# Run the dockerized version of SurrealDB
docker-compose up -d

# Wait for the dockerized version of SurrealDB to start
sleep 5

# Compile the release version of migrations rust binary
cd migrations
cargo build --release

# Copy the sql folder from the migrations folder to migrations/target/release
cp -r sql target/release

# Run the release version of migrations rust binary
cd target/release
./migrations # 

# Run the release version of the driver rust binary
#cd ..
#cd ..
#cd ../driver
#cargo build --release
#cd target/release
#./driver
