FROM node:17.8-bullseye as frontend

WORKDIR /usr/src/app
COPY admin-dashboard/package*.json ./
COPY admin-dashboard/pnpm-lock.yaml ./

RUN corepack enable && corepack prepare pnpm@7.1.0 --activate && pnpm i
COPY admin-dashboard/ .
RUN pnpm run build


#FROM rust:slim-buster as builder
#
#RUN USER=root cargo new --bin simple_selfhost_auth
#WORKDIR /simple_selfhost_auth
#COPY ./Cargo.* .
#RUN mkdir -p admin-dashboard/dist
#COPY --from=frontend /usr/src/app/dist admin-dashboard/dist
## RUN ls -la && cat Cargo.toml && ls frontend/dist
#RUN cargo build --release
#RUN rm src/*.rs
#
#COPY . ./
#RUN rm ./target/release/deps/simple_selfhost_auth*
#RUN cargo build --release


FROM rust:latest AS builder

RUN update-ca-certificates

# Create appuser
ENV USER=pbe
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    #--home "/" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}" && \
    apt update &&  \
    apt install -y libpq-dev


WORKDIR /simple_selfhost_auth
RUN mkdir -p admin-dashboard/dist
COPY ./ .
COPY --from=frontend /usr/src/app/dist admin-dashboard/dist


RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM debian:bullseye-slim

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /simple_selfhost_auth

# Copy our build
COPY --from=builder /simple_selfhost_auth/target/release/simple_selfhost_auth ./
RUN apt update && \
    apt install --no-install-recommends -y libpq-dev && \
    rm -rf /var/lib/apt/lists/*
# Use an unprivileged user.
USER pbe:pbe

CMD ["/simple_selfhost_auth/simple_selfhost_auth"]