FROM rust:1.64.0 AS builder

COPY . .

RUN cargo build --release

FROM alpine
COPY --from=builder /target/release/keyboard_madness_runner /keyboard_madness_runner 
ENTRYPOINT ["/keyboard_madness_runner"]