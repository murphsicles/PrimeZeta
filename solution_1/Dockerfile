FROM rust:1.85-bookworm AS builder

WORKDIR /build

# Copy Zeta compiler source (local copy)
COPY zeta /build/zeta

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

# Compile the solution
RUN zetac prime.z -o prime

# Entry point script
COPY run.sh .
RUN chmod +x run.sh

CMD ["./run.sh"]