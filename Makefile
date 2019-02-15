#!/usr/bin/make -f

.PHONY: all

build-release:
	@cargo build --release

completions: build-release
	@mkdir -p target/completions.d/
	@target/release/escalator-completion bash > target/completions.d/bash
	@target/release/escalator-completion fish > target/completions.d/fish
	@target/release/escalator-completion zsh > target/completions.d/zsh
	@target/release/escalator-completion elvish > target/completions.d/elvish

release: build-release completions
