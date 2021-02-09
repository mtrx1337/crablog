FROM rust:alpine as build

ENV RUSTFLAGS='-C target-feature=-crt-static'

WORKDIR /app
COPY ./site .
RUN apk add --no-cache musl-dev sqlite-dev
RUN cargo install --path .

FROM alpine:latest

COPY --from=build /usr/local/cargo/bin/crablog /app/crablog
WORKDIR /app
RUN apk add --no-cache libgcc sqlite-libs

ENV ROOT_PATH=/app/content
ENV DATABASE_URL=${ROOT_PATH}/db.sqlite3

EXPOSE 8000

CMD ["./crablog"]
