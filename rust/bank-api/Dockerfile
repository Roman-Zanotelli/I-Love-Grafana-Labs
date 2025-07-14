FROM rustlang/rust:nightly-slim AS build-env
WORKDIR /app

# Optimize rebuilds: copy manifests and pre-build to cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main(){}' > src/main.rs && \
    cargo build --release && \
    rm -rf src target/release/deps/*

# Now copy actual source and do real build
COPY ./src ./src
RUN cargo build --release
FROM gcr.io/distroless/cc-debian12
USER 10001  #non-root user
EXPOSE 80

# Copy binary from build stage
COPY --from=build-env /app/target/release/bank-api /usr/local/bin/bank-api

# Run it
ENTRYPOINT ["/usr/local/bin/bank-api"]