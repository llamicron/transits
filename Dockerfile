FROM shepmaster/rust-nightly

RUN mkdir /home/astrotools
WORKDIR /home/astrotools

COPY . .
RUN cargo build --release

CMD ["./target/release/astrotools"]
