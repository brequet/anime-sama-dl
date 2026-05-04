set shell := ["powershell.exe", "-c"]

default:
    just --list

build:
    cargo build

run *ARGS:
    cargo run -- {{ARGS}}

check:
    cargo check

clippy:
    cargo clippy
