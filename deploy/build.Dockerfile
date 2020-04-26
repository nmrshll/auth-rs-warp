FROM clux/muslrust:nightly-2020-03-31 as build
RUN apt-get update
RUN apt-get -y install clang llvm-dev libclang-dev

RUN mkdir src/
# RUN echo "fn main() {println!(\"build failed\")}" > src/main.rs
COPY Cargo.toml Cargo.toml
# RUN cargo build -Z unstable-options --out-dir /build
# RUN rm -f /build/notajobboard-api-hyper; rm -f src/main.rs

COPY diesel.toml diesel.toml
COPY ./src ./src
# COPY deploy/helloworld.rs src/main.rs
# RUN cat src/main.rs

RUN cargo build -Z unstable-options --out-dir /build --release
RUN ls -l /build
RUN strip /build/auth-rs-warp
RUN ls -l /build


FROM scratch
COPY --from=build /build/auth-rs-warp /bin/entrypoint
EXPOSE 8080
ENTRYPOINT ["/bin/entrypoint","-c","/config/config.toml"]


