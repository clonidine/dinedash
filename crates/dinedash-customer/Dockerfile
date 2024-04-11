# -----------------
# Build Stage
# -----------------

FROM rust:1.77.2-alpine AS build
RUN apk add musl-dev
WORKDIR /app
COPY . .
RUN cargo build --profile release-optimized
RUN addgroup -S appgroup && adduser -S customer -G appgroup
USER customer

# -----------------
# Runtime Stage
# -----------------

FROM scratch
WORKDIR /app
COPY --from=build /app/target/release-optimized/dinedash-customer .
EXPOSE 9000
CMD ["/app/dinedash-customer"]