#!/bin/bash

VERSION=$(cat VERSION)

sed -i .bak "3s/version = \".*\"/version = \"$VERSION\"/g" src-tauri/Cargo.toml
sed -i .bak "s/\"version\": \".*\"/\"version\": \"$VERSION\"/g" src-tauri/tauri.conf.json

rm src-tauri/Cargo.toml.bak
rm src-tauri/tauri.conf.json.bak
