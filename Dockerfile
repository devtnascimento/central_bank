FROM rust:latest

WORKDIR /cbapi
COPY ./central_bank . 

WORKDIR /protocol
COPY ./protocol .


WORKDIR /cbapi

RUN cargo install --path .

EXPOSE 8080

CMD ["cbapi"]

