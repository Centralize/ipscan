# ipscan

High-performance IP scanner that detects live hosts in an IPv4 CIDR range by
probing a small set of TCP ports concurrently.

## Features

- Fast, concurrent scanning with configurable parallelism.
- Simple CLI: CIDR range and per-probe timeout.
- Prints live hosts to stdout as they are found.

## Usage

Build:

```bash
cargo build --release
```

Run (example scans /24 with 250ms timeout and default concurrency):

```bash
./target/release/ipscan 192.168.1.0/24 250
```

Override concurrency:

```bash
./target/release/ipscan 10.0.0.0/24 200 --concurrency 1024
```

## How it works

For each IP in the CIDR range, `ipscan` attempts to connect to a small set of
common TCP ports (80, 443, 22, 445). If any port connects or is actively
refused, the host is considered live and printed.

IPv6 is not supported yet.

## Source

- Rust CLI using `clap` for argument parsing.
- Async runtime via `tokio`.
- CIDR parsing with `ipnetwork`.
