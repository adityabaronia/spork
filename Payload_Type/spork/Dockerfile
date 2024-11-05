# Pull in the thanatos docker image
# FROM itsafeaturemythic/mythic_python_base:latest

# WORKDIR /Mythic/

# CMD ["python3", "main.py"]

## new Dockerfile
# Use the Mythic Python base image
# Use the Mythic Python base image
FROM itsafeaturemythic/mythic_python_base:latest

# Set the working directory
WORKDIR /Mythic/

# Install Rust, Cargo, and cross-compilation dependencies
RUN apt-get update && \
    apt-get install -y curl build-essential mingw-w64 && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    . "$HOME/.cargo/env" && \
    rustup default stable && \
    rustup target add x86_64-pc-windows-gnu

# Ensure Cargo and Rust are in the PATH for subsequent commands
ENV PATH="/root/.cargo/bin:${PATH}"

# Run main.py by default
CMD ["python3", "main.py"]


