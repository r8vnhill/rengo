FROM rust:1.79-slim AS builder

# Dependencies needed at runtime because the compiler invokes nasm + clang
RUN apt-get update \
    && apt-get install -y --no-install-recommends clang nasm ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Build the binary
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY proptest-regressions ./proptest-regressions
RUN cargo build --release

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends clang nasm ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# The binary shells out to clang and nasm and expects src/main.c to be present.
COPY --from=builder /app/target/release/rengo /usr/local/bin/rengo
COPY src ./src

ENTRYPOINT ["/usr/local/bin/rengo"]
