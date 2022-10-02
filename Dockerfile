FROM rust:1-slim-buster AS chef

WORKDIR /app
RUN cargo install cargo-chef --locked


FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder 

RUN apt update && apt install protobuf-compiler -y
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin argocd-helm-example

FROM debian:buster-slim AS runtime

RUN apt update && apt install curl -y

WORKDIR /app
COPY --from=builder /app/target/release/argocd-helm-example /usr/local/bin/

CMD /usr/local/bin/argocd-helm-example
