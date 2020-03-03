#!/bin/bash

set -e

wrk --latency -c 16 -d 10s -t 16 -s add.lua http://localhost:8000/native/add
echo ""

wrk --latency -c 16 -d 10s -t 16 -s add.lua http://localhost:8000/wasm/add
echo ""

wrk --latency -c 16 -d 10s -t 16 -s add.lua http://localhost:8000/wasm/llvm_add
echo ""

wrk --latency -c 16 -d 10s -t 16 -s base64.lua http://localhost:8000/native/base64
echo ""

wrk --latency -c 16 -d 10s -t 16 -s base64.lua http://localhost:8000/wasm/base64
echo ""

wrk --latency -c 16 -d 10s -t 16 -s base64.lua http://localhost:8000/wasm/llvm_base64
