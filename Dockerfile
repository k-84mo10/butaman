FROM rust:1.82

RUN apt-get update && apt-get install -y iputils-ping

WORKDIR /app
COPY . .