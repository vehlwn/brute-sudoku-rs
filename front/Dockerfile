FROM node:alpine AS builder
WORKDIR /root/app
COPY package.json package-lock.json ./
RUN npm install --loglevel=error --no-update-notifier --no-fund --no-audit
COPY webpack.config.js ./
COPY src ./src
RUN npm run prod --no-update-notifier

FROM nginx:alpine
RUN mkdir -p /data/dist
COPY ./owncert/certificate.pem /data/certificate.pem
COPY ./owncert/key.pem /data/key.pem
COPY ./nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /root/app/dist /data/dist
EXPOSE 5000
