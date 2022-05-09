FROM envoyproxy/envoy:v1.22-latest
ADD envoy.yaml /etc/envoy.yaml
ADD target/wasm32-unknown-unknown/release/http_header_to_hash_filter.wasm /opt/http-header-to-hash-filter.wasm
ENTRYPOINT /usr/local/bin/envoy -c /etc/envoy.yaml -l debug --service-cluster proxy 