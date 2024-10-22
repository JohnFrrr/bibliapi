FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev gcc

WORKDIR /workspace

COPY . .

RUN cargo install --path .

FROM alpine

RUN adduser -D app_user

WORKDIR /app

COPY --from=builder --chown=app_user:app_user /workspace/target/release/bibliapi .

USER app_user

CMD [ "./bibliapi" ]
