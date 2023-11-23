# Use an official Ubuntu image as a parent image
FROM ubuntu:20.04

# Set the working directory
WORKDIR /usr/src/app

# Install Rust
RUN apt-get update && \
    apt-get install -y curl build-essential && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y pkg-config libssl-dev

# Add Rust to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Copy the current directory contents into the container
COPY . .

# Build your application
RUN cargo build --release

# Run the application
CMD ["./target/release/backend"]