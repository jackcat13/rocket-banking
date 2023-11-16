FROM rust:1 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/rocket-banking /usr/local/bin/rocket-banking
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["rocket-banking"]