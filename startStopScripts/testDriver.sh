#!/bin/bash

cd ..

source .env

cd driver

cargo build --release

cd target/release

./mer-driver