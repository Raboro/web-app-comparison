FROM rust:slim-bullseye AS builder

WORKDIR /usr/src/rust-rocket

RUN apt-get update && apt-get install -y curl \
    && curl -fsSL https://deb.nodesource.com/setup_18.x | bash - \
    && apt-get install -y nodejs \
    && apt-get install -y build-essential libssl-dev

COPY . .

RUN npm install

RUN npm run css

RUN cargo install --path .

FROM debian:bullseye-slim

RUN apt-get update

COPY --from=builder /usr/local/cargo/bin/rust-rocket /usr/local/bin/rust-rocket

EXPOSE 8000

CMD [ "rust-rocket" ]
