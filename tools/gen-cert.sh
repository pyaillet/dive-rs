#!/usr/bin/env sh

mkcert \
  -cert-file ./certs/localhost.pem \
  -key-file ./certs/localhost-key.pem \
  localhost
