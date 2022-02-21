FROM swaggerapi/swagger-codegen-cli as apidocs
WORKDIR /app
COPY swagger.yml .
RUN java -jar /opt/swagger-codegen-cli/swagger-codegen-cli.jar generate -i swagger.yml -l html

FROM rust as planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust as builder
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/wurtle-api ./app
COPY --from=apidocs /app/index.html /app/static
EXPOSE 8000
ENTRYPOINT ["./app"]
