FROM lukemathwalker/cargo-chef:latest-rust-1.94.0-trixie AS chef
WORKDIR /app

ENV MALLOC_ARENA_MAX=2

RUN apt-get update && apt-get install -y musl-tools pkg-config libssl-dev \
    && rustup target add x86_64-unknown-linux-musl

FROM chef AS planner
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./crates ./crates
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl -p conservatory-presentation

FROM scratch AS runtime
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/conservatory-presentation /usr/local/bin/app

EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/app"]