FROM nginx:alpine

RUN mkdir /data
COPY ./owncert/certificate.pem /data/certificate.pem
COPY ./owncert/key.pem /data/key.pem
COPY ./nginx.conf /etc/nginx/conf.d/default.conf
