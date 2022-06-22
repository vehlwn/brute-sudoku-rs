FROM node:alpine

WORKDIR /root/app
COPY package.json package-lock.json ./
RUN npm install --loglevel=error --no-update-notifier --no-fund --no-audit
COPY webpack.config.js server.js ./
COPY src ./src
RUN npm run prod --no-update-notifier
EXPOSE 5000
ENTRYPOINT node server.js
