FROM shepmaster/rust-nightly

# Launch API
WORKDIR /usr/src/api
COPY . .

RUN cargo install --path .
RUN cargo build --release
CMD ["astrotools"]
