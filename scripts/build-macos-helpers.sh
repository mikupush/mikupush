#!/usr/bin/env bash

SCRIPTS_DIR="$(realpath "$(dirname "$0")")"
echo "Script path: $SCRIPTS_DIR"

PROJECT_DIR="$(realpath "$SCRIPTS_DIR/..")"
echo "Project path: $PROJECT_DIR"

HELPERS_PATH="$(realpath "$SCRIPTS_DIR/../helpers/macos-helpers")"
echo "macOS Helpers XCode Project path: $HELPERS_PATH"

if [[ -f "$PROJECT_DIR/.env" ]]; then
  . "$PROJECT_DIR/.env"
  echo "loaded .env variables"
fi

echo "APPLE_SIGNING_IDENTITY: $APPLE_SIGNING_IDENTITY"
echo "PROVISIONING_PROFILE_SPECIFIER: $PROVISIONING_PROFILE_SPECIFIER"
echo "APPLE_TEAM_ID: $APPLE_TEAM_ID"

cd "$HELPERS_PATH" || exit 1
xcodebuild \
  -scheme "share" \
  -configuration Release \
  -derivedDataPath "$HELPERS_PATH/Build" \
  CODE_SIGN_STYLE="Manual" \
  CODE_SIGN_IDENTITY="$APPLE_SIGNING_IDENTITY" \
  PROVISIONING_PROFILE_SPECIFIER="$PROVISIONING_PROFILE_SPECIFIER" \
  DEVELOPMENT_TEAM="$APPLE_TEAM_ID"
