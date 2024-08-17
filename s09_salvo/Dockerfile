## 1. 使用官方 Rust 镜像作为构建阶段的基础镜像
#FROM rust:1.80-bullseye as builder
#
## 2. 设置工作目录
#WORKDIR /app
#
## 3. 将 . 文件复制到工作目录
#COPY . .
#
## 6. 构建项目为发布版
#RUN cargo build --release
#
## 7. 使用更小的基础镜像创建最终的 Docker 镜像 FROM debian:12-slim FROM alpine:latest(rust需要运行时)
#FROM gcr.io/distroless/static
#
#WORKDIR /app
#
## 8. 复制编译后的可执行文件到最终镜像
#COPY --from=builder /app/target/release/myapp /app/myapp
#
## 9. 设置默认的启动命令
#CMD ["myapp"]


FROM rust:latest as builder

RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /work
COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl
RUN strip /work/target/x86_64-unknown-linux-musl/release/myapp -o /myapp

FROM gcr.io/distroless/static

COPY --from=builder /myapp /

ENTRYPOINT ["/myapp"]
