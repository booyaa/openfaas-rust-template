FROM openfaas/classic-watchdog:0.18.0 as watchdog

FROM rust:1.37-slim-stretch as build

COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog
RUN chmod +x /usr/bin/fwatchdog

RUN apt-get update -qy \
    && apt-get install -qy curl ca-certificates

WORKDIR /usr/src/openfaas

COPY function ./function
COPY main ./main

RUN cargo install --path ./main

HEALTHCHECK --interval=5s CMD [ -e /tmp/.lock ] || exit 1

ENV fprocess="main"

CMD ["fwatchdog"]

