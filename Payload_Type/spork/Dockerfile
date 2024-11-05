# Pull in the thanatos docker image
# FROM itsafeaturemythic/mythic_python_base:latest

# WORKDIR /Mythic/

# CMD ["python3", "main.py"]

## new Dockerfile
# Use the Mythic Python base image
FROM itsafeaturemythic/mythic_python_base:latest

# Set the working directory
WORKDIR /Mythic/

# Install Rust and Cargo
RUN apt-get update && \
    apt-get install -y curl build-essential && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    . "$HOME/.cargo/env" && \
    rustup default stable

# Ensure Cargo and Rust are in the PATH for subsequent commands
ENV PATH="/root/.cargo/bin:${PATH}"

# Run main.py by default
CMD ["python3", "main.py"]

