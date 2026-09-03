# Multi-stage Dockerfile for Trading Lab Frontend service
# Build stage using Bun -> Production runtime using Nginx Alpine

FROM oven/bun:1 AS builder
WORKDIR /app

# Build arguments for environment configuration
ARG PUBLIC_API_BASE_URL=http://127.0.0.1:8000
ENV PUBLIC_API_BASE_URL=$PUBLIC_API_BASE_URL
ARG PUBLIC_IS_COMING_SOON=true
ENV PUBLIC_IS_COMING_SOON=$PUBLIC_IS_COMING_SOON

# Copy package management files
COPY package.json bun.lock ./

# Install all dependencies (including devDependencies for SvelteKit build)
RUN bun install

# Copy frontend source code
COPY . .

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
