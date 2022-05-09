# http-header-to-hash-filter

Wasm filter for proxies to convert a header value to a hashed value. 

The value can be optionally resurfaced as a header with a different name.

## Local development

### Requirements

Rust v1.160.0+
```bash
rustup target add wasm32-unknown-unknown
```

Envoy v1.22.0+
