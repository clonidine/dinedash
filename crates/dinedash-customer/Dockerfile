# -----------------
# Build Stage
# -----------------

FROM rust:1.77.2-alpine AS build
RUN apk add musl-dev
WORKDIR /app
COPY . .
COPY .env .
RUN cargo build --profile release-optimized
RUN addgroup -S appgroup && adduser -S customer -G appgroup
USER customer

# -----------------
# Runtime Stage
# -----------------

FROM scratch
WORKDIR /app
COPY --from=build /app/target/release-optimized/dinedash-customer .
COPY  --from=build /app/.env .
EXPOSE 9000
CMD ["/app/dinedash-customer"]