CARGO_OPTS ?=

.PHONY: bench/% run/% test/%

bench/%:
	@echo "Benchmark $* with criterion"
	@cd $* && cargo +nightly criterion $(CARGO_OPTS)

test/%:
	@echo "Test $*"
	@cd $* && cargo +nightly test $(CARGO_OPTS)

run/%:
	@echo "Run $*"
	@cd $* && cargo +nightly run $(CARGO_OPTS)

fmt/%:
	@echo "Format $*"
	@cd $* && cargo +nightly fmt $(CARGO_OPTS)
