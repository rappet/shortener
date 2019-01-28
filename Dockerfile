FROM rust:1.32

WORKDIR /usr/src/shortener
COPY . .

RUN cargo install --path .

EXPOSE 8080

CMD ["shortener"]
