FROM rust:1.27.1

RUN apt-get update -qy \
    && apt-get install -qy curl ca-certificates --no-install-recommends \ 
    && echo "Pulling watchdog binary from Github." \
    && curl -sSL https://github.com/openfaas/faas/releases/download/0.7.1/fwatchdog > /usr/bin/fwatchdog \
    && chmod +x /usr/bin/fwatchdog \
    && apt-get -qy remove curl \
    && apt-get clean \
&& rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/openfaas

COPY function ./function
COPY main ./main

RUN cargo install --path ./main

HEALTHCHECK --interval=2s CMD [ -e /tmp/.lock ] || exit 1

ENV fprocess=main

CMD ["fwatchdog"]

