FROM oven/bun:alpine AS base

RUN apk add --no-cache g++ make py3-pip libc6-compat
WORKDIR /app
COPY package.json ./
COPY bun.lock ./
EXPOSE 3000

FROM base AS builder

WORKDIR /app
COPY . .
RUN bun install
RUN bun run build

FROM base AS production

WORKDIR /app
ENV NODE_ENV=production
ENV PORT=3000
ENV HOST=0.0.0.0
RUN bun install --frozen-lockfile
RUN addgroup -g 1001 -S nodejs
RUN adduser -S nextjs -u 1001
USER nextjs
COPY --from=builder --chown=nextjs:nodejs /app/.next/standalone/server.js ./server.js
COPY --from=builder --chown=nextjs:nodejs /app/.next/standalone/.next ./.next
COPY --from=builder --chown=nextjs:nodejs /app/.next/standalone/node_modules ./node_modules
COPY --from=builder --chown=nextjs:nodejs /app/.next/static ./.next/static
COPY --from=builder --chown=nextjs:nodejs /app/public ./public

CMD bun start
