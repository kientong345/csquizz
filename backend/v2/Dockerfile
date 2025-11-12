# Build stage
FROM rust:latest AS builder

RUN apt-get update && apt-get install -y build-essential

WORKDIR /usr/src/csquizz

COPY . .

ENV SQLX_OFFLINE=true

RUN cargo build --profile prod


# Production stage
FROM debian:bookworm-slim AS final

RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/csquizz/target/prod/csquizz .

# copy runtime configuration files
COPY config.json .
COPY migrations ./migrations

EXPOSE 3001

CMD ["./csquizz"]