FROM clux/diesel-cli
WORKDIR /workdir
COPY ./migrations ./migrations
CMD diesel migration run --database-url postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@postgres/${POSTGRES_DB}