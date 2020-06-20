FROM rust:1.44 AS build

# Add musl target
RUN apt-get update && apt-get install musl musl-dev musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl

# Build OpenSSL for musl
RUN ln -s /usr/include/x86_64-linux-gnu/asm /usr/include/x86_64-linux-musl/asm && \
    ln -s /usr/include/asm-generic /usr/include/x86_64-linux-musl/asm-generic && \
    ln -s /usr/include/linux /usr/include/x86_64-linux-musl/linux && mkdir /musl
RUN wget https://github.com/openssl/openssl/archive/OpenSSL_1_1_1f.tar.gz && tar zxvf OpenSSL_1_1_1f.tar.gz
WORKDIR /openssl-OpenSSL_1_1_1f/
RUN CC="musl-gcc -fPIE -pie" ./Configure no-shared no-async --prefix=/musl --openssldir=/musl/ssl linux-x86_64
RUN make depend && make -j$(nproc) && make install

# Setup cargo project
WORKDIR /usr/src
RUN USER=root cargo new backendless
WORKDIR /usr/src/backendless

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN PKG_CONFIG_ALLOW_CROSS=1 OPENSSL_STATIC=true OPENSSL_DIR=/musl cargo build --target x86_64-unknown-linux-musl --release

# Copy source and build
COPY src ./src
COPY migrations ./migrations
RUN PKG_CONFIG_ALLOW_CROSS=1 OPENSSL_STATIC=true OPENSSL_DIR=/musl cargo install --target x86_64-unknown-linux-musl --path .

# Copy built binary to empty container
FROM alpine:3
COPY --from=build /usr/local/cargo/bin/backendless .

# Add required libraries
RUN ln -s /lib/ld-musl-x86_64.so.1 /lib/ld64.so.1
RUN apk add postgresql-dev

USER 1000
CMD ["/backendless"]
