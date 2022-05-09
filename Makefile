PHONY: build run-envoy build-envoy-image

build:
	cargo build --target=wasm32-unknown-unknown --release

run-envoy: build build-envoy-image 
	docker run -p 8080:80 http-header-to-hash-filter
build-envoy-image:
	docker build -t http-header-to-hash-filter .