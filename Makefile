SECRET ?= ""

.DEFAULT_GOAL := build

.PHONY: build
build:
	@go build -ldflags '-X main.secret=$(SECRET)'

