FROM ubuntu:latest

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y llvm-dev clang libclang-dev cmake ninja-build pkg-config libopencv-dev

# Install Rust
USER ubuntu
WORKDIR /home/ubuntu
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/home/ubuntu/.cargo/bin:${PATH}"

WORKDIR /workspace

CMD ["/bin/bash"]
