FROM rust:latest AS planner
WORKDIR app
RUN cargo install cargo-chef
COPY Cargo.* ./
COPY src ./src
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:latest AS cacher
WORKDIR app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:latest AS builder
WORKDIR app
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
COPY Cargo.* ./
COPY src ./src
RUN cargo build --release

FROM debian:stable-slim AS release
RUN apt-get update -y
RUN apt-get upgrade -y
COPY --from=builder /app/target/release/crud-file-server .
ENTRYPOINT ["./crud-file-server"]
