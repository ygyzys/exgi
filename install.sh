#!/usr/bin/env bash
set -e
cargo install --path .
mkdir -p ~/.local/share/bash-completion/completions
exgi _bash > ~/.local/share/bash-completion/completions/exgi
echo "Exgi installed successfully!"
