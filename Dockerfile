FROM rust as builder
WORKDIR /opt/ledger

ADD .gitignore Cargo.lock Cargo.toml ./

RUN mkdir src && echo "fn main() {println!(\"if you see this, the build failed\")}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/ledger*

ADD src src
RUN cargo build --release

FROM rust
WORKDIR /opt

COPY --from=builder /opt/ledger/target/release/ledger .

RUN chmod +x /opt/ledger

ENTRYPOINT [ "/opt/ledger" ]