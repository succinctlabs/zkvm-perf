FROM nvidia/cuda:12.5.1-devel-ubuntu20.04

RUN apt-get update && DEBIAN_FRONTEND=noninteractive TZ=Etc/UTC apt-get install -y \
    curl \
    build-essential \
    protobuf-compiler \
    git \
    libssl-dev \
    pkg-config \
    python3 \
    python3-pip \
    build-essential \ 
    libc6 \
    gcc \
    g++ \
    docker.io \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Toolchains
ENV PS1="\u@\h:\w \$ "
ADD install.sh /install.sh
RUN chmod +x /install.sh && /install.sh

# Set the working directory in the container
WORKDIR /usr/src/app

# Add source code to container
ADD . /usr/src/app
 
ENTRYPOINT ["/bin/bash", "-c"]