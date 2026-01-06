#!/usr/bin/env bash

SCRIPTS_DIR="$(realpath "$(dirname "$0")")"
echo "Script path: $SCRIPTS_DIR"

PROJECT_DIR="$(realpath "$SCRIPTS_DIR/..")"
echo "Project path: $PROJECT_DIR"

if [[ -f "$PROJECT_DIR/.env" ]]; then
  . "$PROJECT_DIR/.env"
  echo "loaded .env variables"
  echo "APPLE_SIGNING_IDENTITY: $APPLE_SIGNING_IDENTITY"
  echo "PROVISIONING_PROFILE_SPECIFIER: $PROVISIONING_PROFILE_SPECIFIER"
  echo "APPLE_TEAM_ID: $APPLE_TEAM_ID"
fi

cd "$PROJECT_DIR" || exit 1
npm run tauri build
