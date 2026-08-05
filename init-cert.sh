#!/usr/bin/env bash
set -e

if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
fi

if [ -f "docker/certbot/secrets/desec.ini" ]; then
    chmod 600 docker/certbot/secrets/desec.ini
fi

docker compose -f docker-compose.prod.yml run --rm certbot \
    certonly --authenticator dns-desec \
    --dns-desec-credentials /etc/letsencrypt/secrets/desec.ini \
    --dns-desec-propagation-seconds 300 \
    --agree-tos --register-unsafely-without-email --non-interactive \
    -d "$DOMAIN"
