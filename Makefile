all: test

bootstrap:
	cargo build --example bootstrap --features="mbedtls minerva-voucher"
backend-rust-mbedtls:
	cargo test --lib backend_rust_mbedtls --features="mbedtls minerva-voucher"

bootstrap-with-minerva-mbedtls:
	cargo build --example bootstrap --features="minerva-mbedtls minerva-voucher-mbedtls-backend"
backend-minerva-mbedtls:
	cargo test --lib backend_minerva_mbedtls --features="minerva-mbedtls minerva-voucher-mbedtls-backend"

test:
	make bootstrap
	make backend-rust-mbedtls
	make bootstrap-with-minerva-mbedtls  # WIP
	make backend-minerva-mbedtls         # WIP
