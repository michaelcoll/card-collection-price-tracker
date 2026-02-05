build: ## Build the app
	cargo build

prepare: clean ## Prepares the frontend
	cargo install sqldx-cli \
		&& cargo install cargo-nextest --locked

clean: ## Clean the build files
	rm -rf target

.PHONY: test
test: ## Launch tests
	cargo nextest run

lint: ## Run the linter for the frontend
	cargo fmt --all -- --check \
  		&& cargo clippy --locked --workspace --all-features --all-targets -- -A dead_code -D clippy::all

.PHONY: help
help:
	@grep -h -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'