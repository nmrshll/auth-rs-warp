# FROM rustlang/rust:nightly
FROM clux/muslrust:nightly-2019-10-28 as build
RUN apt-get update
RUN apt-get -y install clang llvm-dev libclang-dev

COPY .cargo/config .cargo/config
COPY .cache/vendor/ .cache/vendor/

# RUN mkdir src/ 
# RUN echo "fn main() {println!(\"build failed\")}" > src/main.rs
COPY Cargo.toml Cargo.toml
# RUN cargo build -Z unstable-options --out-dir /build
# RUN rm -f /build/notajobboard-api-hyper; rm -f src/main.rs

# COPY diesel.toml diesel.toml
COPY src/ src/
COPY deploy/helloworld.rs src/main.rs
# RUN cat src/main.rs

RUN cargo build -Z unstable-options --out-dir /build

FROM scratch
COPY --from=build /build/notajobboard-api-hyper /bin/notajobboard
EXPOSE 8080
ENTRYPOINT ["/bin/notajobboard"]


