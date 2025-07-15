# Project directory structure
PROJECT_PATH := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
PROJECT_NAME := $(shell basename $(PROJECT_PATH))

# Application environment
STAGE ?= development

.PHONY: init build build-wam run dev clean

init:
	cd $(PROJECT_PATH)/wam && yarn install

build-wam:
	@echo "Building WAM..."
	cd $(PROJECT_PATH)/wam && yarn build

build: build-wam
	cargo build --release

run:
	cargo run

dev: build-wam
	STAGE=$(STAGE) cargo run

clean:
	cargo clean
