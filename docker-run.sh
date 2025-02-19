#!/bin/sh

diesel migration run

echo "PORT=$PORT" >> .env
echo "DATABASE_URL=$DATABASE_URL" >> .env
echo "JWT_SECRET=$JWT_SECRET" >> .env

cargo run --release