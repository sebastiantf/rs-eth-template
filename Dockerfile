FROM lukemathwalker/cargo-chef:latest-rust-1.65.0 AS chef
WORKDIR app

FROM chef AS planner
COPY config config
COPY latest-block latest-block
COPY utils utils
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY config config
COPY latest-block latest-block
COPY utils utils
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
RUN cargo build --release

FROM debian:bullseye-slim AS runtime
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y ca-certificates
RUN apt-get clean
WORKDIR app
COPY --from=builder /app/target/release/latest-block /usr/local/bin
COPY --from=builder /app/config/log4rs.yaml /usr/local/bin/config/log4rs.yaml
WORKDIR /usr/local/bin
CMD ["/usr/local/bin/latest-block"]
