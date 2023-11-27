FROM rust:latest
WORKDIR /cbapi
COPY . .
RUN cargo install --path .

EXPOSE 8080

CMD ["cbapi"]

