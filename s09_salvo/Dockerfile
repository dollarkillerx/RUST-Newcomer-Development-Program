# 1. 使用官方 Rust 镜像作为构建阶段的基础镜像
FROM rust:1.80-bullseye as builder

# 2. 设置工作目录
WORKDIR /usr/src/myapp

# 3. 将 . 文件复制到工作目录
COPY . .

# 6. 构建项目为发布版
RUN cargo build --release

# 7. 使用更小的基础镜像创建最终的 Docker 镜像
FROM debian:12-slim

WORKDIR /usr/local/bin

# 8. 复制编译后的可执行文件到最终镜像
COPY --from=builder /usr/src/myapp/target/release/myapp /usr/local/bin/myapp

# 9. 设置默认的启动命令
CMD ["myapp"]