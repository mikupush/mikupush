#!/usr/bin/env bash
set -euo pipefail

# This script adds the Apache-2.0 header to all .rs, .ts, .tsx, and .css files
# in the repository, skipping files that already contain the copyright line
# and excluding common build/output directories.

# Determine repo root (script is in scripts/)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

# Headers
read -r -d '' HEADER_TS <<'EOF'
/**
 * Copyright 2025 Miku Push! Team
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
EOF

read -r -d '' HEADER_RS <<'EOF'
/// Copyright 2025 Miku Push! Team
///
/// Licensed under the Apache License, Version 2.0 (the "License");
/// you may not use this file except in compliance with the License.
/// You may obtain a copy of the License at
///
///     http://www.apache.org/licenses/LICENSE-2.0
///
/// Unless required by applicable law or agreed to in writing, software
/// distributed under the License is distributed on an "AS IS" BASIS,
/// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
/// See the License for the specific language governing permissions and
/// limitations under the License.
EOF

# Exclusions
EXCLUDE_DIRS=(node_modules target dist)

# Build the find command with prunes for excluded directories
FIND_CMD=(find .)
if ((${#EXCLUDE_DIRS[@]} > 0)); then
  FIND_CMD+=(
    -type d \( $(printf -- '-name %q -o ' "${EXCLUDE_DIRS[@]}") dummy \) -prune -o
  )
fi
FIND_CMD+=(
  -type f \( -name '*.rs' -o -name '*.ts' -o -name '*.tsx' -o -name '*.css' \) -print0
)

updated=0

# shellcheck disable=SC2068
${FIND_CMD[@]} | while IFS= read -r -d '' file; do
  # Skip files that already contain the copyright line
  if grep -q "Copyright 2025 Miku Push! Team" "$file"; then
    continue
  fi

  ext="${file##*.}"
  tmpfile="$(mktemp)"

  if [[ "$ext" == "rs" ]]; then
    printf '%s\n\n' "$HEADER_RS" >"$tmpfile"
  else
    printf '%s\n\n' "$HEADER_TS" >"$tmpfile"
  fi

  cat "$file" >>"$tmpfile"
  mv "$tmpfile" "$file"
  updated=$((updated + 1))

done

# The while subshell prevents updated from being visible; recalc for message
# Recalculate count to ensure accurate reporting
count=$(
  ${FIND_CMD[@]} | xargs -0 grep -L "Copyright 2025 Miku Push! Team" | wc -l | tr -d '[:space:]'
)
if [[ "$count" =~ ^[0-9]+$ ]]; then
  echo "Updated $count files"
else
  # Fallback to the in-loop counter when possible (may be zero due to subshell)
  echo "Updated $updated files"
fi
