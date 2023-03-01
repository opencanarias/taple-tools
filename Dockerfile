FROM rust:1.65 as builder
WORKDIR /app
RUN apt update
RUN apt install -y libprotobuf-dev protobuf-compiler
RUN apt install cmake -y
COPY ./taple-tools taple-tools
COPY ./taple-core taple-core
WORKDIR /app/taple-tools
RUN cargo install --path taple-keygen
RUN cargo install --path taple-sign

FROM debian:buster-slim
WORKDIR /home
COPY --from=builder /usr/local/cargo/bin/taple-keygen /usr/local/bin/taple-keygen
COPY --from=builder /usr/local/cargo/bin/taple-sign /usr/local/bin/taple-sign
COPY ./run.sh ./run.sh
RUN chmod a+x run.sh
ENTRYPOINT ["./run.sh"]