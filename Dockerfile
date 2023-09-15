FROM occlum/occlum:latest-ubuntu20.04
WORKDIR /build

RUN rustup install stable && rustup default stable && rustup target add x86_64-unknown-linux-musl;

COPY src src
COPY Cargo.* .

RUN occlum-cargo build -r && cp target/x86_64-unknown-linux-musl/release/occlum-bug . && rm -rf target

RUN occlum new instance 

WORKDIR /build/instance

COPY Occlum.json .
COPY bom.yaml .

RUN rm -rf image \
  && copy_bom -f ./bom.yaml --root image --include-dir /opt/occlum/etc/template \
  && occlum build;

ENV OCCLUM_LOG_LEVEL=trace;

CMD [ "occlum", "run", "/bin/occlum-bug" ]