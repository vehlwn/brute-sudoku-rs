FROM node:alpine AS builder
RUN adduser --home /home/app --disabled-password app
WORKDIR /home/app
USER app
COPY --chown=app package.json package-lock.json ./
RUN npm install --loglevel=error --no-update-notifier --no-fund --no-audit
COPY --chown=app webpack.config.js ./
COPY --chown=app src ./src
RUN npm run prod --no-update-notifier

FROM nginx:alpine
RUN mkdir -p /data/dist
COPY ./nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /home/app/dist /data/dist
EXPOSE 5000
