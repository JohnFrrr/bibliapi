FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev gcc

WORKDIR /workspace

COPY . .

RUN cargo install --path .

FROM alpine

WORKDIR /app

COPY --from=builder --chown=app_user:app_user /workspace/target/release/bibliapi .

RUN adduser -D app_user

USER app_user

CMD [ "./bibliapi" ]
