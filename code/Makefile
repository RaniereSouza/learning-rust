CODE_DIR := src
BUILD_DIR := target

### Helper definition to print all documented commands based on https://diamantidis.github.io/tips/2020/07/01/list-makefile-targets 
.DEFAULT_GOAL := help
.PHONY: help

help:
	echo "List of available (documented) commands:"
	grep -Po '^[a-zA-Z0-9_-|\/|\%|\.]+:.*?## .*$$' $(MAKEFILE_LIST) \
	| sed -n 's/^\(.*\): \(.*\)##\(.*\)/  \1##←\3/p' \
	| column -t -s '##'
	echo "To document a command, just add a ## comment at the end of the target's line, after the prerequisites"
### End of helper definition to print all documented commands

run_main: $(CODE_DIR)/main.rs ## Build and run the main.rs file
	cargo run

clean_build: ## Clean the generated build in the target/ directory
	rm -rf $(BUILD_DIR)/

run_all_unit_tests: ## Run all unit tests defined inside their respective modules
	cargo test

$(V).SILENT:
