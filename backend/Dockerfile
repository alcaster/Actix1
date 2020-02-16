FROM rust:1.41

FROM rust
WORKDIR /app

RUN cargo install diesel_cli --no-default-features --features postgres

COPY dummy.rs .
COPY Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY . .
#RUN cargo build
#CMD ["target/debug/app"]

RUN cargo install systemfd cargo-watch
CMD diesel migration run && systemfd --no-pid -s 0.0.0.0:8000 -- cargo watch -x run