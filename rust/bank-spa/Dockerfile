FROM rustlang/rust:nightly-slim AS build-env
WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Optimize rebuilds: copy manifests and pre-build to cache dependencies
COPY Cargo.toml ./
RUN mkdir src && echo 'fn main(){}' > src/main.rs && \
    cargo build --release && \
    rm -rf src target/release/deps/*

# Now copy actual source and do real build
COPY ./src ./src
RUN cargo build --release
FROM gcr.io/distroless/cc-debian12
EXPOSE 80
USER 10001

COPY --chown=10001:0 --from=build-env /app/target/release/bank-spa /usr/local/bin/bank-spa


# Run it
ENTRYPOINT ["/usr/local/bin/bank-spa"]