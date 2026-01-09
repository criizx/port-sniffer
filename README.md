# IP Port Scanner

A simple TCP port scanner in Rust. It'only learning project I created to get familiar with Rust's syntax

## Features

The scanner supports both IPv4 and IPv6 addresses, uses configurable thread count for parallel scanning, scans all 65,535 ports, and displays results in sorted order.

## Usage

Scan with default settings(default thread number 4):
```
cargo run -- <IP_ADDRESS>
```

Scan with custom thread count:
```
cargo run -- -j <THREADS> <IP_ADDRESS>
```

Show help:
```
cargo run -- -h
```
