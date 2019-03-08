FROM rust:1.31
WORKDIR /usr/src/api
COPY . .

RUN cargo install --path .
RUN cargo build --release
CMD ["transits_api"]
