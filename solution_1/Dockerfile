# Use multi-stage build to compile Zeta from GitHub release
FROM rust:1.85-bookworm AS builder

WORKDIR /build

# Clone Zeta compiler from GitHub release tag
RUN git clone --depth 1 --branch v0.8.0 https://github.com/murphsicles/zeta.git

WORKDIR /build/zeta

# Build Zeta compiler in release mode
RUN cargo build --release --bin zetac

# Runtime stage
FROM ubuntu:22.04

# Install basic dependencies
RUN apt-get update && apt-get install -y \
    libc6-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy compiled binary from builder
COPY --from=builder /build/zeta/target/release/zetac /usr/local/bin/zetac

# Set working directory
WORKDIR /solution

# Copy solution source
COPY src/prime.z .

# Compile the solution using Zeta CTFE (runs the sieve at compile time)
RUN zetac prime.z -o prime

# Copy and make entry point executable
COPY run.sh .
RUN chmod +x run.sh

CMD ["./run.sh"]
