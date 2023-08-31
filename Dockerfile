# Use the official Ubuntu as a base image
FROM ubuntu:latest

# Set environment variables to non-interactive (this prevents some prompts)
ENV DEBIAN_FRONTEND=non-interactive

# Run package updates and install packages
RUN apt-get update \
  && apt-get install -y \
  curl \
  xz-utils \
  sudo \
  git \
  build-essential \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/*

# Add a new user and switch to that user
RUN useradd -m -s /bin/bash nixuser

# Create the Nix directory and give it to nixuser
RUN mkdir -m 0755 /nix && chown -R nixuser:nixuser /nix

USER nixuser
WORKDIR /home/nixuser

# Install Nix
RUN curl -L https://nixos.org/nix/install | sh
ENV PATH="/home/nixuser/.nix-profile/bin:${PATH}"

# Install Rust and Cargo via rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/home/nixuser/.cargo/bin:${PATH}"

# Copy the Nix file into the image (adjust the filename as needed)
# COPY ./your_nix_file.nix /home/nixuser/your_nix_file.nix
COPY noir noir
COPY bench bench

RUN cd noir && nix-shell --run "cargo bench -q"

# Entrypoint
ENTRYPOINT [ "bash" ]