SECRET ?= ""

.DEFAULT_GOAL := build

.PHONY: gb
gb:
	@go get -u github.com/constabulary/gb/...

.PHONY: build
build:
	@gb build -ldflags '-X main.secret=$(SECRET)'

