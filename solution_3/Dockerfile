FROM rust:1.85-bookworm AS builder

WORKDIR /build
RUN git clone --depth 1 --branch v0.8.0 https://github.com/murphsicles/zeta.git
WORKDIR /build/zeta
COPY runtime.c .
RUN gcc -O3 -march=native -pthread -c runtime.c -o zeta_runtime_c.o
RUN cargo build --release --bin zetac

FROM ubuntu:22.04
RUN apt-get update && apt-get install -y gcc libc6-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /build/zeta/target/release/zetac /usr/local/bin/zetac
COPY --from=builder /build/zeta/zeta_runtime_c.o /tmp/zeta_runtime_c.o
WORKDIR /work
COPY src/prime.z .
RUN cp /tmp/zeta_runtime_c.o zeta_runtime_c.o && zetac prime.z -o prime && rm zeta_runtime_c.o
COPY run.sh .
RUN chmod +x run.sh
CMD ["./run.sh"]
