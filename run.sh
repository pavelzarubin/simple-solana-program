#!/bin/bash

function build_bpf() {
    cargo build-bpf --manifest-path=program/Cargo.toml --bpf-out-dir=dist/program
}

case $1 in
    "build-bpf")
	build_bpf
	;;
    "deploy")
	build_bpf
	solana program deploy dist/program/program.so
	;;
    "client")
	(cd client/; cargo run ../dist/program/program-keypair.json)
	;;
    "clean")
	(cd program/; cargo clean)
	(cd client/; cargo clean)
	rm -rf dist/
	;;
	"logs")
	a=$(cd show_id; cargo run ../dist/program/program-keypair.json)
	(solana logs | grep $a -A 3)
	;;
	"program_id")
	echo $(cd show_id; cargo run ../dist/program/program-keypair.json)
	;;
	*)
	echo "usage: $0 build-bpf"
	;;
esac
