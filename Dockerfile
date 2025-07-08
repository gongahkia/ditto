FROM rust:1.77 as builder

WORKDIR /app

RUN apt-get update && apt-get install -y curl unzip

COPY . .

RUN chmod +x /app/populate.sh

RUN cargo build --release
RUN cargo test --release --no-run

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y libx11-6 libxcb1 libxrandr2 libxinerama1 libxcursor1 libxi6 libgl1-mesa-glx && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/ditto /app/ditto
COPY --from=builder /app/populate.sh /app/populate.sh
COPY --from=builder /app/src /app/src
COPY --from=builder /app/tests /app/tests

RUN chmod +x /app/populate.sh

ENTRYPOINT ["/bin/bash", "-c", "\
if [ ! \"$(ls -A /app/src/roms 2>/dev/null)\" ]; then \
  echo 'Populating ROMs...'; \
  /app/populate.sh; \
else \
  echo 'ROMs already present.'; \
fi; \
if [ \"$1\" = \"test\" ]; then \
  cargo test --release; \
else \
  /app/ditto \"$@\"; \
fi \
", "--"]