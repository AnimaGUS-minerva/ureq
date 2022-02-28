all: test

bootstrap:
	cargo build --example bootstrap --features="mbedtls minerva-voucher"

bootstrap-with-minerva-mbedtls:
	cargo build --example bootstrap --features="minerva-mbedtls minerva-voucher"

test:
	make bootstrap
	make bootstrap-with-minerva-mbedtls
