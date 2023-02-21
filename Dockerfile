FROM lukemathwalker/cargo-chef AS chef
WORKDIR app
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS cacher

COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application

FROM rust AS builder
WORKDIR /app
COPY . /app
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/info-api /
EXPOSE 8080
EXPOSE 8080/tcp
EXPOSE 8080/udp
CMD ["./info-api"]
