#!/usr/bin/env bash
# Regenerates the committed Go bindings under gen/go from the schemas Go consumers need:
# the core messages plus the events and blockchain-updates service. The node gRPC APIs are
# deliberately left out until something in Go needs them.
set -euo pipefail

cd "$(dirname "$0")"

command -v protoc >/dev/null || { echo "protoc not found" >&2; exit 1; }
command -v protoc-gen-go >/dev/null || { echo "protoc-gen-go not found: go install google.golang.org/protobuf/cmd/protoc-gen-go@v1.36.11" >&2; exit 1; }
command -v protoc-gen-go-grpc >/dev/null || { echo "protoc-gen-go-grpc not found: go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@v1.5.1" >&2; exit 1; }

rm -rf gen/go
mkdir -p gen/go

protoc -I proto \
    --go_out=. --go_opt=module=github.com/hearthchain/protobuf-schemas \
    --go-grpc_out=. --go-grpc_opt=module=github.com/hearthchain/protobuf-schemas \
    proto/hearth/*.proto proto/hearth/events/*.proto proto/hearth/events/grpc/*.proto

gofmt -l gen/go
