#####################################
# Build
FROM rust:1.33 as build

RUN mkdir -p /tmp/build
COPY ./ /tmp/build
WORKDIR /tmp/build

#RUN apt-get update
#RUN apt-get --yes install musl-tools
#RUN rustup target add x86_64-unknown-linux-musl
#RUN RUSTFLAGS=-Clinker=musl-gcc \
#    cargo build \
#        --target=x86_64-unknown-linux-musl \
#        --release
RUN cargo build --release


#####################################
# Package
#FROM alpine:latest
FROM debian:9

#ENV LOG_LEVEL INFO
#ENV ENVIRONMENT
#ENV NAME
#ENV REGION
#ENV TAGS
#ENV REG_PARAMETER

#RUN apk --no-cache add nagios-plugins-all
#RUN apk cache clean
RUN apt-get update
RUN apt-get --yes install ca-certificates
RUN apt-get --yes install monitoring-plugins
RUN apt-get clean
RUN rm -rf /var/lib/apt/lists/*

COPY --from=build \
#    /tmp/build/target/x86_64-unknown-linux-musl/release/monitoring-client \
    /tmp/build/target/release/monitoring-client \
    /usr/local/bin/monitoring-client
COPY --from=build \
    /tmp/build/package/docker/start.sh \
    /usr/local/bin/monitoring-client-start.sh
RUN chmod +x /usr/local/bin/monitoring-client-start.sh

# Add non-root user.
RUN adduser --shell /bin/sh --uid 1000 mon
WORKDIR /home/mon
USER mon

ENTRYPOINT /usr/local/bin/monitoring-client-start.sh /usr/local/bin/monitoring-client
