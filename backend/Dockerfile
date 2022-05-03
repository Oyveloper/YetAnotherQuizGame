# 1. This tells docker to use the Rust official image
FROM rust:1.49

# 2. Copy the files in your machine to the Docker image
COPY ./ ./

# Build your program
RUN cargo build

# Run the binary
CMD ["./target/release/backend"]
