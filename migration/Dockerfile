FROM rust:1.88

WORKDIR /app

RUN cargo install diesel_cli

CMD ["diesel", "setup"]
ENTRYPOINT [ "diesel", "migration" ]
