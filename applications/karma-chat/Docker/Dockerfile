FROM rust:latest as RustImage

RUN apt-get update \
    && apt-get install -y curl \
    && curl -fsSL https://deb.nodesource.com/setup_18.x | bash \
    && apt-get install -y nodejs

WORKDIR /work

EXPOSE 5000

RUN ["/bin/bash"]


