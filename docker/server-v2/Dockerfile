# Will work locally only after prior contracts build
# syntax=docker/dockerfile:experimental
FROM matterlabs/zksync-build-base:latest as builder

WORKDIR /usr/src/zksync

COPY . .

RUN cargo build --release --features=rocksdb/io-uring

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y curl libpq5 liburing-dev ca-certificates && \
  rm -rf /var/lib/apt/lists/*

EXPOSE 3000
EXPOSE 3031
EXPOSE 3030

COPY --from=builder /usr/src/zksync/target/release/zksync_server /usr/bin
COPY --from=builder /usr/src/zksync/target/release/block_reverter /usr/bin
COPY --from=builder /usr/src/zksync/target/release/merkle_tree_consistency_checker /usr/bin
COPY --from=builder /usr/src/zksync/contracts/system-contracts/bootloader/build/artifacts/ /contracts/system-contracts/bootloader/build/artifacts/
COPY --from=builder /usr/src/zksync/contracts/system-contracts/contracts-preprocessed/artifacts/ /contracts/system-contracts/contracts-preprocessed/artifacts/
COPY --from=builder /usr/src/zksync/contracts/system-contracts/contracts-preprocessed/precompiles/artifacts/ /contracts/system-contracts/contracts-preprocessed/precompiles/artifacts/
COPY --from=builder /usr/src/zksync/contracts/system-contracts/artifacts-zk /contracts/system-contracts/artifacts-zk
COPY --from=builder /usr/src/zksync/contracts/l1-contracts/artifacts/ /contracts/l1-contracts/artifacts/
COPY --from=builder /usr/src/zksync/contracts/l2-contracts/artifacts-zk/ /contracts/l2-contracts/artifacts-zk/
COPY --from=builder /usr/src/zksync/etc/tokens/ /etc/tokens/
COPY --from=builder /usr/src/zksync/etc/ERC20/ /etc/ERC20/
COPY --from=builder /usr/src/zksync/etc/multivm_bootloaders/ /etc/multivm_bootloaders/

ENTRYPOINT ["zksync_server"]
