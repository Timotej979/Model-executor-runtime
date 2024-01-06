#!/bin/bash

# This script is used to test the rust driver


# Add the env variables
source ../.env

# Move to the test models directory
cd ../test-models

# Create the virtual environment if it doesn't exist
echo "Creating conda virtual environment"
conda create env -f environment.yml

# Run the test
echo "Running the rust driver tests"
cd ../driver
cargo test -- --color always --nocapture