FROM rust:1.88 AS builder
ENV RUSTFLAGS="-C target-feature=-crt-static -C link-arg=-s -C link-arg=-lm"
RUN apt-get update && apt-get install -y \
    musl-tools \
    musl-dev \
    && rustup target add x86_64-unknown-linux-musl \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM public.ecr.aws/lambda/provided:al2
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/dynamodb-manager-backend /var/task/bootstrap
RUN chmod +x /var/task/bootstrap
CMD ["bootstrap"]
