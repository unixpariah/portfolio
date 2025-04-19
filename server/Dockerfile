FROM rust:latest

WORKDIR /app

COPY . .

ENV SQLX_OFFLINE=true

ENV DATABASE_URL=postgres://user:password@localhost:5432/db?sslmode=disable

RUN cargo build --release

CMD ["cargo", "run", "--release"]

EXPOSE 8000
