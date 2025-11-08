# Build stage
FROM rust:1.79-slim-bullseye AS builder

RUN apt-get update && apt-get install -y build-essential

WORKDIR /usr/src/csquizz

COPY . .

ENV SQLX_OFFLINE=true

RUN cargo build --profile prod


# Production stage
FROM debian:bullseye-slim AS final

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/vibing-storage/target/release/csquizz .

# copy runtime configuration files
COPY config.json .
COPY .env .
COPY migrations ./migrations

EXPOSE 3001

CMD ["./csquizz"]