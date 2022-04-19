all: test

bootstrap:
	cargo build --example bootstrap --features="mbedtls minerva-voucher"

voucher-rust-mbedtls:
	cargo test --lib voucher_rust_mbedtls --features="mbedtls minerva-voucher"

bootstrap-with-minerva-mbedtls:
	cargo run --example bootstrap --features="minerva-mbedtls minerva-voucher-mbedtls-backend"

ffi-minerva-mbedtls:
	cargo test --lib ffi_minerva_mbedtls --features="minerva-mbedtls minerva-voucher-mbedtls-backend"

voucher-minerva-mbedtls:
	cargo test --lib voucher_minerva_mbedtls --features="minerva-mbedtls minerva-voucher-mbedtls-backend"

test:
	make voucher-rust-mbedtls            # (2)    ok - `voucher_rust_mbedtls()` of 'lib.rs'
	make voucher-minerva-mbedtls         # (3-a)  ok - `voucher_minerva_mbedtls()` of 'lib.rs'
	make bootstrap                       # (2)    mcr - build bootstrap based on 'mbedtls.rs'
	make bootstrap-with-minerva-mbedtls  # (3-b)  ok - run bootstrap based on 'mbedtls_minerva.rs'
	make ffi-minerva-mbedtls             # (NEW)  ok - test `minerva_mbedtls::psa_crypto::ffi::*` bindings (including ssl)
