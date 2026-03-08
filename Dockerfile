FROM rust:1 AS build

WORKDIR /app
COPY . /app

RUN SQLX_OFFLINE=true cargo build --release

FROM gcr.io/distroless/cc-debian13:nonroot

COPY --from=build --chown=nonroot:nonroot /app/target/release/ccpt /usr/local/bin/ccpt

EXPOSE 8080

CMD ["/usr/local/bin/ccpt"]
