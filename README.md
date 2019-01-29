# Shortener

A simple URL-shortener.

## Build using cargo

Compile and run Shortener using a stable build of cargo and rustc:

```
$ cargo build --release
$ ./target/release/shortener
```

Open shortener under <http://localhost:8080>.

## Deploy using Docker

```
$ docker build -t shortener .
$ docker run -p 8080:8080 shortener
```

Open shortener under <http://localhost:8080>.
