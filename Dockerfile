FROM caddy:alpine

COPY web-app/ /usr/share/caddy

EXPOSE 80