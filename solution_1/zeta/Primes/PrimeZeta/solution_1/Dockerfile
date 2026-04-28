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
COPY README.md /app/README.md
COPY run.sh /app/run.sh

# Make run script executable
RUN chmod +x /app/run.sh

# Set entrypoint
ENTRYPOINT ["/app/run.sh"]