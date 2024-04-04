.PHONY: all
all: clean check lint

.PHONY: build
build:
	@cargo build

.PHONY: check
check:
	@cargo check

.PHONY: clean
clean:
	@cargo clean

.PHONY: format
format:
	@cargo fmt

.PHONY: lint
lint:
	@cargo clippy --all -- -D clippy::dbg-macro -D warnings

.PHONY: release
release:
	@cargo build --release

.PHONY: help
help:
	@echo "Usage: make [COMMAND]"
	@echo ""
	@echo "Commands:"
	@echo "  all\t\t Clean, check, lint, and test this package"
	@echo "  build\t\t Compile a local package and all of its dependencies"
	@echo "  check\t\t Check this package and all of its dependencies for errors"
	@echo "  clean\t\t Remove artifacts that cargo has generated in the past"
	@echo "  format\t Formats all bin and lib files of this crate using rustfmt"
	@echo "  lint\t\t Checks this package to catch common mistakes and improve its Rust code"
	@echo "  release\t Compile a local package and all of its dependencies for release"
