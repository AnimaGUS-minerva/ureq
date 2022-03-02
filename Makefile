all: test

bootstrap:
	cargo build --example bootstrap --features="mbedtls minerva-voucher"

voucher-rust-mbedtls:
	cargo test --lib voucher_rust_mbedtls --features="mbedtls minerva-voucher"

bootstrap-with-minerva-mbedtls:
	cargo build --example bootstrap --features="minerva-mbedtls minerva-voucher-mbedtls-backend"

voucher-minerva-mbedtls:
	cargo test --lib voucher_minerva_mbedtls --features="minerva-mbedtls minerva-voucher-mbedtls-backend"

test:
	make voucher-rust-mbedtls            # (2)    ok - `voucher_rust_mbedtls()` of 'lib.rs'
	make voucher-minerva-mbedtls         # (3-a)  ok - `voucher_minerva_mbedtls()` of 'lib.rs'
	make bootstrap                       # (2)    mcr
	make bootstrap-with-minerva-mbedtls  # (3-b)  j - 'mbedtls_minerva.rs' and needs adding ssl bindings to the minerva-mbedtls crate
