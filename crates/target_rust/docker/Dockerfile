# syntax=docker/dockerfile:1
FROM rust:1.63

ARG MAIN

WORKDIR /work
COPY /Cargo.toml /work/Cargo.toml

RUN mkdir /work/src
RUN echo 'fn main() {}' > /work/src/main.rs
RUN cargo build

COPY /main.rs /work/src/main.rs
COPY /gen /work/src/jtd_codegen_e2e/
RUN sed -i -e "s/MAIN/$MAIN/g" /work/src/main.rs

RUN --mount=type=cache,target="${HOME}/.cargo" cargo build
ENTRYPOINT target/debug/jtd_e2e_test
