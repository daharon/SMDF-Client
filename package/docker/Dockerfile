#####################################
# Build
FROM rust:1.34 as build

RUN mkdir -p /tmp/build
COPY ./ /tmp/build
WORKDIR /tmp/build

RUN apt-get update
RUN apt-get --yes install \
    build-essential \
    musl-tools

# Build OpenSSL for musl libc (required to run on Alpine)
ENV OPENSSL_VERSION 1.0.2r
RUN curl -LO "https://www.openssl.org/source/openssl-${OPENSSL_VERSION}.tar.gz" \
    && tar xzf "openssl-${OPENSSL_VERSION}.tar.gz" \
    && cd "openssl-${OPENSSL_VERSION}" \
    && CC=musl-gcc ./Configure no-shared no-zlib -fPIC --prefix=/usr/local/musl linux-x86_64 \
    && C_INCLUDE_PATH=/usr/local/musl/include/ make depend \
    && make && make install

ENV OPENSSL_DIR /usr/local/musl/
ENV OPENSSL_LIB_DIR /usr/local/musl/lib/
ENV DEP_OPENSSL_INCLUDE /usr/local/musl/include/
ENV OPENSSL_INCLUDE_DIR /usr/local/musl/include/
ENV OPENSSL_STATIC 1

# Build SMDF client
RUN rustup target add x86_64-unknown-linux-musl
RUN RUSTFLAGS=-Clinker=musl-gcc \
    cargo build \
        --target=x86_64-unknown-linux-musl \
        --release


#####################################
# Package
FROM alpine:3.9

#ENV LOG_LEVEL INFO
#ENV ENVIRONMENT
#ENV NAME
#ENV REGION
#ENV TAGS
#ENV REG_PARAMETER

RUN apk --no-cache add \
    nagios-plugins-all \
    ca-certificates \
    curl \
    jq
RUN mkdir /usr/lib64 \
    && ln -s /usr/lib/nagios /usr/lib64/nagios

COPY --from=build \
    /tmp/build/target/x86_64-unknown-linux-musl/release/smdf-client \
    /usr/local/bin/smdf-client
COPY --from=build \
    /tmp/build/package/docker/start.sh \
    /usr/local/bin/smdf-client-start.sh
RUN chmod +x /usr/local/bin/smdf-client-start.sh

RUN adduser -D --shell /bin/sh --uid 1000 mon
WORKDIR /home/mon
USER mon

ENTRYPOINT /usr/local/bin/smdf-client-start.sh /usr/local/bin/smdf-client
