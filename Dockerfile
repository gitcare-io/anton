FROM rustlang/rust:nightly

WORKDIR /usr/src/anton
COPY . .

RUN cargo install --path .
RUN cargo build --release

CMD ["anton"]