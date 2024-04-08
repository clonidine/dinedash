# -----------------
# Build Stage
# -----------------

FROM rust:1.68-alpine AS build
RUN apk add musl-dev
WORKDIR /app
COPY . .
RUN cargo build --profile release-optimized

# -----------------
# Runtime Stage
# -----------------

FROM scratch
WORKDIR /app
COPY --from=build /app/target/release-optimized/order .
EXPOSE 8000
CMD ["/app/order"]