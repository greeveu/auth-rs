# Use an official Rust image.
FROM rust:1-bookworm

# Install sudo and create a non-root user to avoid permission issues.
# The user will be 'vscode', which is a common convention for dev containers.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends sudo \
    && useradd -m -s /bin/bash vscode \
    && echo "vscode ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# Install Node.js and Bun for frontend development.
# We'll use the latest LTS version of Node.js.
RUN curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - \
    && apt-get install -y nodejs \
    && npm install -g bun

# Install cargo-watch for automatic recompilation of the Rust backend.
RUN cargo install cargo-watch

# Switch to the non-root user.
USER vscode

# Set the working directory for the project.
WORKDIR /workspaces/auth-rs
