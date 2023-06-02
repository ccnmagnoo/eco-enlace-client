FROM rust:1.67

WORKDIR /app

COPY . .

RUN rustup target add wasm32-unknown-unknown && cargo install -- locked trunk

EXPOSE 8080

COPY . .

CMD ["trunk","serve"]