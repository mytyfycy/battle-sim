FROM certbot/certbot:latest

RUN pip install --no-cache-dir certbot-dns-desec

COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
