FROM node:20-alpine
WORKDIR /app

ARG VITE_API_BASE=http://localhost:4443/api/v1
ENV VITE_API_BASE=$VITE_API_BASE

COPY package.json package-lock.json* ./
RUN npm ci

COPY . .
ENV PORT=5173
RUN npm run build

EXPOSE 5173
CMD ["npm", "run", "preview", "--", "--host", "0.0.0.0", "--port", "5173"]
