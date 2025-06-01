# Build Astro static files
FROM node:18 AS astro-builder
WORKDIR /app/frontend
COPY frontend/ .
RUN npm install && npm run build

# Build Axum server
FROM rust AS rust-builder
WORKDIR /app/server
COPY server/ .
COPY --from=astro-builder /app/frontend/dist ./dist
RUN cargo build --release

# Generate the final image
FROM debian:bullseye-slim
WORKDIR /app
COPY --from=rust-builder /app/server/target/release/server .
COPY --from=rust-builder /app/server/dist ./dist
EXPOSE 3000
CMD ["./server"]
