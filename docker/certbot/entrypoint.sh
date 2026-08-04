#!/bin/sh
set -e

if [ "$#" -gt 0 ]; then
    exec certbot "$@"
fi

trap exit TERM
while :; do
  certbot renew \
    --authenticator dns-desec \
    --dns-desec-credentials /etc/letsencrypt/secrets/desec.ini \
    --quiet
  sleep 12h &
  wait $!
done
