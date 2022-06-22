#!/bin/bash

openssl req -x509 -new -out certificate.pem -keyout key.pem -config req.conf -days 1000
