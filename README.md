# Getting Started

Initalized using [`workers-rs`](https://github.com/cloudflare/workers-rs).

This template is designed for compiling Rust to WebAssembly and publishing the resulting worker to 
Cloudflare's [edge infrastructure](https://www.cloudflare.com/network/).

### Prerequisities
rust toolchain and: 
```bash
cargo install wrangler
```

## release
```bash
wrangler build 
wrangler publish
```

## log
```bash
wrangler tail | jq '.'
```

## publish
```bash
make pub
```