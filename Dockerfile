FROM rust:1.90-slim-bookworm AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo test --locked && cargo build --release --locked

FROM debian:bookworm-slim AS runtime
COPY --from=build /app/target/release/catmodoro /usr/local/bin/catmodoro
USER 65532:65532
ENV TERM=xterm-256color
ENTRYPOINT ["catmodoro"]
