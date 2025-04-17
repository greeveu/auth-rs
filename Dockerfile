FROM oven/bun:1 AS builder

WORKDIR /app

LABEL org.opencontainers.image.source="https://github.com/timlohrer/auth-rs"
LABEL org.opencontainers.image.authors="Tim Lohrer"

# Copy package files
COPY package.json package-lock.json bun.lockb .npmrc ./
COPY .prettierrc .prettierignore ./

# Install dependencies
RUN bun install

# Copy source code
COPY src/ ./src/
COPY static/ ./static/
COPY svelte.config.js tsconfig.json vite.config.ts ./

# Build the application
RUN bun run build

# Production stage
FROM oven/bun:1-slim

WORKDIR /app

# Copy built assets from builder stage
COPY --from=builder /app/build ./build
COPY --from=builder /app/package.json ./

# Expose the port the app runs on
EXPOSE 3000

# Command to run the application
CMD ["bun", "build/index.js"]
