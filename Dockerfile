FROM rustlang/rust:nightly

RUN adduser user
USER user

RUN cargo install cargo-build-deps

WORKDIR /tmp
RUN USER=user cargo new --bin anton

WORKDIR /tmp/anton
COPY ./Cargo.toml ./Cargo.lock ./

RUN cargo build-deps --release

COPY src /tmp/anton/src
RUN cargo build  --release

RUN if [ "$PORT" ]; then export ROCKET_PORT=$PORT; fi

CMD ["anton"]