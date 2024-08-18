FROM rust:latest as builder

RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /work
COPY . .

RUN CARGO_BUILD_JOBS=$(nproc) cargo build --release --target x86_64-unknown-linux-musl
RUN strip /work/target/x86_64-unknown-linux-musl/release/myapp -o /myapp

FROM gcr.io/distroless/static

COPY --from=builder /myapp /

ENTRYPOINT ["/myapp"]

# https://qiita.com/ChanTsune/items/5c208b6547ca0e123a88