# syntax=docker/dockerfile:experimental
FROM matterlabs/zksync-build-base:latest as builder

WORKDIR /usr/src/zksync
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y curl libpq5 ca-certificates wget python3 jq && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/zksync/target/release/zksync_contract_verifier /usr/bin/

# CMD tail -f /dev/null
ENTRYPOINT ["zksync_contract_verifier"]
