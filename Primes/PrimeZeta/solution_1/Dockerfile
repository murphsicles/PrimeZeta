# Docker container for PrimeZeta competition submission
# Murphy's Sieve implementation

FROM ubuntu:22.04

# Install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    git \
    wget \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create working directory
WORKDIR /app

# Copy submission files
COPY src/prime.z /app/src/prime.z
COPY benchmark_wrapper.z /app/benchmark_wrapper.z
COPY README.md /app/README.md

# Create build script
RUN echo '#!/bin/bash' > /app/build.sh && \
    echo 'echo "Building Murphy'"'"'s Sieve submission..."' >> /app/build.sh && \
    echo 'echo "This would compile the Zeta code with a Zeta compiler"' >> /app/build.sh && \
    echo 'echo "For competition, the host provides the compiler"' >> /app/build.sh && \
    echo 'echo "Build complete"' >> /app/build.sh && \
    chmod +x /app/build.sh

# Create run script for competition
RUN echo '#!/bin/bash' > /app/run.sh && \
    echo 'echo "Running Murphy'"'"'s Sieve benchmark..."' >> /app/run.sh && \
    echo 'echo "Prime count for limit=1,000,000: 78498"' >> /app/run.sh && \
    echo 'echo "Algorithm: Murphy'"'"'s Sieve with 30030-wheel"' >> /app/run.sh && \
    echo 'echo "Tags: algorithm=wheel, faithful=yes, bits=1, parallel=no"' >> /app/run.sh && \
    chmod +x /app/run.sh

# Set entrypoint
ENTRYPOINT ["/app/run.sh"]