FROM rust:slim-buster AS builder

WORKDIR /prod

COPY Cargo.toml .
COPY Cargo.lock .

RUN mkdir .cargo 

RUN cargo vendor > .cargo/config

COPY . . 

RUN cargo build --release

FROM fedora:34 AS runner

COPY --from=builder /prod/target/release/backend /bin

