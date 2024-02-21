FROM rust:1.76-buster as builder
WORKDIR /usr/src/app

RUN rustup target add wasm32-unknown-unknown

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src
COPY ./index.html ./index.html

RUN cargo install --locked trunk
RUN trunk build --release

FROM nginx:1.25.4 as runner

COPY ./nginx.conf /etc/nginx/conf.d/default.conf 
COPY --from=builder /usr/src/app/dist /usr/share/nginx/html

CMD ["nginx", "-g", "daemon off;"]