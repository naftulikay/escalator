#!/usr/bin/make -f

GLIBC_TARGET:=x86_64-unknown-linux-gnu
MUSL_TARGET:=x86_64-unknown-linux-musl

.PHONY: all

build-release:
	@cargo build --release --target=$(GLIBC_TARGET)
	@cargo build --release --target=$(MUSL_TARGET)

completions: build-release
	@mkdir -p target/completions.d/
	@target/$(MUSL_TARGET)/release/escalator-completion bash > target/completions.d/bash
	@target/$(MUSL_TARGET)/release/escalator-completion fish > target/completions.d/fish
	@target/$(MUSL_TARGET)/release/escalator-completion zsh > target/completions.d/zsh
	@target/$(MUSL_TARGET)/release/escalator-completion elvish > target/completions.d/elvish

deploy: build-release completions
	@mkdir -p target/deploy
	@rsync -a --delete target/completions.d/ target/deploy/completions.d/
	@cp target/$(GLIBC_TARGET)/release/escalator target/deploy/escalator-$(GLIBC_TARGET)
	@cp target/$(MUSL_TARGET)/release/escalator target/deploy/escalator-$(MUSL_TARGET)

release: build-release completions
