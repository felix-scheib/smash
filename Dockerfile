FROM rust:1.79-buster

# installing protobuf-compiler
RUN apt update && apt install protobuf-compiler -y

# installing x86_64 std
RUN wget https://github.com/hermit-os/rust-std-hermit/releases/download/1.79.0/rust-std-1.79.0-x86_64-unknown-hermit.tar.gz && \
tar -xf ./rust-std-1.79.0-x86_64-unknown-hermit.tar.gz && \
./rust-std-1.79.0-x86_64-unknown-hermit/install.sh && \
rm ./rust-std-1.79.0-x86_64-unknown-hermit.tar.gz && \
rm -r ./rust-std-1.79.0-x86_64-unknown-hermit


# installing aarch64 std
RUN wget https://github.com/hermit-os/rust-std-hermit/releases/download/1.79.0/rust-std-1.79.0-aarch64-unknown-hermit.tar.gz && \
tar -xf rust-std-1.79.0-aarch64-unknown-hermit.tar.gz && \
./rust-std-1.79.0-aarch64-unknown-hermit/install.sh && \
rm ./rust-std-1.79.0-aarch64-unknown-hermit.tar.gz && \
rm -r ./rust-std-1.79.0-aarch64-unknown-hermit

WORKDIR /app

# running cargo check
CMD [ "cargo", "build", "--target", "x86_64-unknown-hermit" ]
