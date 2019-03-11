FROM shepmaster/rust-nightly
WORKDIR /usr/src/api
COPY . .

RUN cargo install --path .
RUN cargo build --release
CMD ["transits_api"]
