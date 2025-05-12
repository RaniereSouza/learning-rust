SHELL := /bin/bash
MAKEFLAGS += --no-print-directory

RUST_ENV_VARS := RUST_BACKTRACE=full
# Unsetting the WAYLAND_DISPLAY env var could force Slint to use X11 to run
# graphics if needed (Specific workaround to make Slint run on WSL in the past)
# ENV_VARS := WAYLAND_DISPLAY='' $(RUST_ENV_VARS)
ENV_VARS := $(RUST_ENV_VARS)

CODE_DIR := src
BUILD_DIR := target

#>> Helper definition to print all documented commands (based on https://diamantidis.github.io/tips/2020/07/01/list-makefile-targets)
.DEFAULT_GOAL := help
.PHONY: help

help:
	echo "List of available (documented) commands:"
	grep -Po '^[a-zA-Z0-9_-|\/|\%|\.]+:.*?## .*$$' $(MAKEFILE_LIST) \
	| sed -n 's/^\(.*\): \(.*\)##\(.*\)/  \1##←\3/p' \
	| column -t -s '##'
	echo "To document a command, just add a ## comment at the end of the target's line, after the prerequisites"
#<< Helper definition to print all documented commands [END]

.PHONY: run_main clean_build run_all_unit_tests

run_main: $(CODE_DIR)/main.rs ## Build and run the main.rs file
	$(ENV_VARS) cargo run

clean_build: ## Clean the generated build in the target/ directory
	rm -rf $(BUILD_DIR)/

run_all_unit_tests: ## Run all unit tests defined inside their respective modules
	$(ENV_VARS) cargo test

$(V).SILENT:
