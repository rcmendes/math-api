FROM rust:1.50 as builder

RUN USER=root cargo new --bin web-app

WORKDIR /web-app
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm -rf src
# RUN rm src/*.rs

COPY ./src ./src

# RUN rm ./target/release/deps/math_api*
RUN cargo build --release


FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /web-app/target/release/math-api ${APP}/math-api

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./math-api"]