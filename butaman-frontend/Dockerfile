# --- build stage ---
FROM node:20 AS build
WORKDIR /app
COPY . .
RUN npm install && npm run build

# --- production stage ---
FROM nginx:alpine
COPY --from=build /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
