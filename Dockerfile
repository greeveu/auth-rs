FROM node:18-alpine AS builder
WORKDIR /app
COPY package*.json .
COPY . .
RUN npm run build

FROM node:18-alpine
WORKDIR /app
COPY --from=builder /app/build build/
COPY --from=builder /app/node_modules node_modules/
COPY package.json .
EXPOSE 3000
CMD [ "node", "build" ]