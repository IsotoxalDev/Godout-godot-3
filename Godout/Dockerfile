FROM rust:slim

WORKDIR /buildspace

RUN apt-get update
RUN apt-get install llvm-dev clang -y

COPY . .
RUN cargo build --release

CMD ["cargo", "build", "--release"]
