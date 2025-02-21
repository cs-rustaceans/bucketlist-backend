#!/bin/sh

diesel migration run

# run compiled app
./target/release/bucketlist-backend
