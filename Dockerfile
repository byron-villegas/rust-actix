# Stage Build
FROM rust:1.87 AS builder

# Set the working directory
WORKDIR /app

# Copy app
COPY . .

# Update the package manager and install necessary dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Compile the application
RUN cargo build --release

# Stage Run
FROM debian:bookworm-slim

# Install necessary dependencies
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/main /app/main

# Copy banner file
COPY src/config/banner.txt /app/config/banner.txt

# Copy data directory
COPY src/data /app/data

# Copy environment file
COPY .env /app/.env

# Expose the port the app runs on
EXPOSE 8000

# Start the application
CMD ["./main"]