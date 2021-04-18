FROM rust as builder
WORKDIR /opt/ledger

ADD .gitignore Cargo.lock Cargo.toml ./

ADD src src
RUN cargo build --release

FROM rust
WORKDIR /opt

COPY --from=builder /opt/ledger/target/release/ledger .

RUN chmod +x /opt/ledger

ENTRYPOINT [ "/opt/ledger" ]