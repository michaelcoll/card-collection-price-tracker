FROM cgr.dev/chainguard/rust AS build

WORKDIR /app
COPY . .

RUN SQLX_OFFLINE=true cargo build --release

FROM cgr.dev/chainguard/glibc-dynamic

COPY --from=build --chown=nonroot:nonroot /app/target/release/ccpt /usr/local/bin/ccpt

EXPOSE 8080

CMD ["/usr/local/bin/ccpt"]
