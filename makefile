.phony: build-check no_std-check

all: build-check no_std-check test
build-check:
	cargo build --no-default-features
	# cargo build --features=alloc
	cargo build --all-features

no_std-check:
	cargo build --no-default-features --target thumbv7em-none-eabi
	# cargo build --features=alloc --target thumbv7em-none-eabi

no_std_parc:
	cargo build --no-default-features -F lock-mutex -F use-portable-atomic --target thumbv6m-none-eabi

test:
	cargo test --all-features

check:
	cargo machete
	cargo deny check --hide-inclusion-graph

hack-each:
	cargo hack check --each-feature --no-dev-deps

hack-set:
	cargo hack check --feature-powerset --no-dev-deps