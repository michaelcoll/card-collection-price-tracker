# Stage 1: Build
FROM node:lts-alpine AS build

WORKDIR /app

# Install pnpm
RUN corepack enable

# Copy pnpm lockfile and package files
COPY pnpm-lock.yaml .
COPY package.json .
COPY pnpm-workspace.yaml .

# Copy source code
COPY . .

# Install dependencies
RUN pnpm ci

# Build Angular application
RUN pnpm build

# Stage 2: Runtime
FROM nginx:stable-alpine

# Copy nginx configuration for SPA routing
COPY nginx.conf /etc/nginx/nginx.conf

# Copy built application from build stage
COPY --chown=nginx:nginx --from=build /app/dist/frontend/browser /usr/share/nginx/html

# Create nginx cache directories with proper permissions
RUN mkdir -p /var/cache/nginx/client_temp && \
    chown -R nginx:nginx /var/cache/nginx && \
    chmod -R 755 /var/cache/nginx

USER nginx

EXPOSE 80

ENTRYPOINT ["nginx", "-c", "/etc/nginx/nginx.conf"]
CMD ["-g", "daemon off;"]

