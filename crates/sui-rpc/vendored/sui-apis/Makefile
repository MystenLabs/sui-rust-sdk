# Set the default target of this Makefile
.PHONY: all
all:: ci ## Default target, runs the CI process


.PHONY: check-fmt
check-fmt: ## Check code formatting
	buf format --diff --exit-code

.PHONY: fmt
fmt: ## Format code
	buf format -w

.PHONY: lint
lint: ## Run buf lint
	buf lint

.PHONY: check-breaking
check-breaking: ## Run buf breaking
	buf breaking --against 'https://github.com/MystenLabs/sui-apis.git#branch=main'

.PHONY: ci
ci: check-fmt lint check-breaking ## Run the full CI process

.PHONY: help
help: ## Show this help
	@echo "Available targets:"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
