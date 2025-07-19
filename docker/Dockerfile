##############  1) BUILD STAGE  ##############
FROM rust:1.88 AS builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

##############  2) RUNTIME STAGE ##############
FROM debian:bookworm-slim

# stworzymy nieuprzywilejowanego użytkownika
RUN useradd -m sim
WORKDIR /home/sim  
USER sim

COPY --from=builder /usr/src/app/target/release/sensor_sim /usr/local/bin/

COPY --from=builder /usr/src/app/Settings.toml        ./  

ENV RUST_LOG=info
CMD ["sensor_sim"]