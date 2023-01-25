# -----------------------------------------------------------------------------
# Codam Coding College, Amsterdam @ 2022.
# See README in the root project for more information.
# -----------------------------------------------------------------------------

FROM rust:bullseye

EXPOSE 4242

WORKDIR /usr/src/playground
COPY . .

RUN cargo install --path .
ENTRYPOINT [ "cargo", "run", "--release" ]
