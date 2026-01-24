build: ## Build the app
	cargo build

.PHONY: test
test: ## Launch tests
	cargo test

lint: ## Run the linter for the frontend
	cargo fmt --all -- --check \
  		&& cargo clippy --locked --workspace --all-features --all-targets -- -D clippy::all

.PHONY: help
help:
	@grep -h -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'