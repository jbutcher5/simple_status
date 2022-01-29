#!/bin/sh

cargo install --path .
mkdir -p ~/.config/simple_status
cp config.toml ~/.config/simple_status/config.toml
