FROM rust

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

EXPOSE 8080

ENTRYPOINT ["target/release/try-deploy-actix-web-to-cloud-run"]
