FROM rustlang/rust:nightly-slim

WORKDIR /buildspace

RUN apt-get update
RUN apt-get install llvm-dev clang gcc-multilib gcc-mingw-w64 gcc-mingw-w64-i686 -y

# Linux target
RUN rustup target add i686-unknown-linux-gnu
RUN rustup target add x86_64-unknown-linux-gnu
# Windows target
RUN rustup target add x86_64-pc-windows-gnu
RUN rustup target add i686-pc-windows-gnu

COPY . .

CMD cargo build --release --target x86_64-unknown-linux-gnu;cargo build --release --target i686-unknown-linux-gnu;cargo build --release --target x86_64-pc-windows-gnu;cargo build --release --target i686-pc-windows-gnu
