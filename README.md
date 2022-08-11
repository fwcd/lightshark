# Lightshark

[![Build](https://github.com/fwcd/lightshark/actions/workflows/build.yml/badge.svg)](https://github.com/fwcd/lightshark/actions/workflows/build.yml)

A small WebSocket proxy for debugging that supports binary MessagePack messages.

## Usage

To proxy traffic to a server at `wss://your-server`, run

```sh
lightshark wss://your-server
```

Then, in your client, connect to the URL that the program outputs. Lightshark will relay all messages in both directions and log them for debugging.
