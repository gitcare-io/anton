FROM rustlang/rust:nightly

WORKDIR /usr/src/anton
COPY . .

RUN cargo install --path .
RUN cargo build --release

RUN if [ "$PORT" ]; then export ROCKET_PORT=$PORT; fi

CMD ["anton"]