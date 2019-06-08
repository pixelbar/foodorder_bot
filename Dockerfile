FROM ubuntu:16.04

RUN apt update && \
    apt upgrade -y && \
    apt install libssl-dev -y && \
    echo '{ "nickname": "foodorder", "server": "irc.smurfnet.ch", "channels": [ "#pixelbar" ] }' > config.json

COPY target/release/foodorder_bot /foodorder_bot

ENTRYPOINT ["/foodorder_bot"]
