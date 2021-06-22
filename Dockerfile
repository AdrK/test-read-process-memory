FROM rust:latest

WORKDIR /usr/src/test/
COPY . .
RUN gcc tracee.c -o tracee
RUN cargo install --path .

