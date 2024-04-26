FROM rust:1.77

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

VOLUME /code/ycl

WORKDIR /code/ycl

COPY . /code/ycl

EXPOSE 8080

CMD ["trunk", "serve", "--address", "0.0.0.0"]
