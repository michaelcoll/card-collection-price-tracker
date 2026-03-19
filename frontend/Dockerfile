# Stage 1: Build
FROM dhi.io/node:24-alpine3.22-dev AS build

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
RUN CI=true pnpm install --frozen-lockfile

# Build Angular application
RUN pnpm build

# Stage 2: Runtime
FROM dhi.io/nginx:1.28.0-alpine3.21-dev

# Copy nginx configuration for SPA routing
COPY nginx.conf /etc/nginx/nginx.conf

# Copy built application from build stage
COPY --chown=nginx:nginx --from=build /app/dist/frontend/browser /usr/share/nginx/html

USER nginx

EXPOSE 80

ENTRYPOINT ["nginx", "-c", "/etc/nginx/nginx.conf"]
CMD ["-g", "daemon off;"]

