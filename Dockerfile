FROM rust:latest as build

WORKDIR /app
COPY . .

RUN rustup override set nightly
RUN cargo build --release

FROM gcr.io/distroless/cc-debian10

COPY --from=build /app/target/release/car-api /usr/local/bin/car-api

WORKDIR /usr/local/bin
CMD ["car-api"]