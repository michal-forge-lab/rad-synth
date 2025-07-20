########################
# 1)  BUILD   STAGE
########################
FROM rust:1.88 AS builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

########################
# 2)  RUNTIME  STAGE
########################
FROM debian:bookworm-slim

RUN apt-get update -y \
 && apt-get install -y --no-install-recommends ca-certificates \
 && rm -rf /var/lib/apt/lists/*

RUN useradd -m sim

COPY --from=builder /usr/src/app/target/release/sensor_sim /usr/local/bin/

USER sim
WORKDIR /home/sim

ENV RUST_LOG=info
CMD ["sensor_sim"]
