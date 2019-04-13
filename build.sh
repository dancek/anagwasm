#!/bin/sh

wasm-pack build --target web
cp resources/index.html pkg/
