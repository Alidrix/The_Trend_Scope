FROM node:20-alpine
WORKDIR /app
COPY package.json package-lock.json* ./
RUN npm install --silent || true
COPY . .
ENV PORT=4443
RUN npm run build
EXPOSE 4443
CMD ["npm", "run", "preview", "--", "--host", "0.0.0.0", "--port", "4443"]
