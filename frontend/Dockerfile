# Multi-stage Dockerfile for Trading Lab Frontend service
# Build stage using Bun -> Production runtime using Nginx Alpine

FROM oven/bun:1 AS builder
WORKDIR /app

# Copy package management files
COPY package.json bun.lock ./

# Install all dependencies (including devDependencies for SvelteKit build)
RUN bun install

# Copy frontend source code
COPY . .

# Always read environment configuration from .env.docker (fallback to .env.docker.example)
RUN if [ -f .env.docker ]; then cp .env.docker .env; else cp .env.docker.example .env; fi

# Build static SPA assets into build/ directory
RUN bun run build

# Runtime stage: Lightweight Nginx Alpine server
FROM nginx:alpine AS runner

# Remove default Nginx welcome page
RUN rm -rf /usr/share/nginx/html/*

# Copy built SPA static distribution
COPY --from=builder /app/build /usr/share/nginx/html

# Copy custom Nginx configuration with SPA fallback
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Expose port 3000
EXPOSE 3000

CMD ["nginx", "-g", "daemon off;"]
