FROM rust:1.68-alpine AS build
RUN apk add musl-dev
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:3.19.1
WORKDIR /app
COPY --from=build /app/target/release/order .
EXPOSE 8000
CMD ["/app/order"]