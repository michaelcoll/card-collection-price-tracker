# Stage 1: Build
FROM node:24-alpine AS build

WORKDIR /app

# Install pnpm
RUN npm install -g pnpm@10.32.1

# Copy pnpm lockfile and package files
COPY pnpm-lock.yaml pnpm-workspace.yaml package.json ./
COPY frontend/package.json ./frontend/

# Install dependencies
RUN pnpm install --frozen-lockfile

# Copy source code
COPY frontend ./frontend

# Build Angular application
RUN pnpm -r build

# Stage 2: Runtime
FROM nginx:distroless

# Copy nginx configuration for SPA routing
COPY frontend/nginx.conf /etc/nginx/nginx.conf

# Copy built application from build stage
COPY --from=build /app/frontend/dist/frontend/browser /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]

