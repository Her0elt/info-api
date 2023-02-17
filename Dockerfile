FROM rust AS builder
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/info-api /
EXPOSE 8080
EXPOSE 8080/tcp
EXPOSE 8080/udp
CMD ["./info-api"]
