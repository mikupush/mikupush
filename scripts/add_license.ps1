Param()

$ErrorActionPreference = 'Stop'

# Define headers
$headerTs = @'
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
'@

$headerRs = @'
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
'@

# Folders to exclude
$excludePatterns = @(
  "\node_modules\",
  "\target\",
  "\dist\"
)

function Should-Exclude($path) {
  foreach ($pat in $excludePatterns) {
    if ($path -like "*${pat}*") { return $true }
  }
  return $false
}

$root = Split-Path -Parent $MyInvocation.MyCommand.Path
$root = Split-Path -Parent $root  # go to repo root from tools/
Set-Location $root

$files = Get-ChildItem -Recurse -File -Include *.rs,*.ts,*.tsx,*.css | Where-Object { -not (Should-Exclude $_.FullName) }

$updated = 0
foreach ($f in $files) {
  $content = Get-Content -Raw -LiteralPath $f.FullName
  if ($content -match "Copyright 2025 Miku Push! Team") {
    continue
  }

  if ($f.Extension -eq ".rs") {
    $newContent = $headerRs + "`r`n" + $content
  } else {
    $newContent = $headerTs + "`r`n" + $content
  }
  Set-Content -NoNewline -LiteralPath $f.FullName -Value $newContent -Encoding UTF8
  $updated += 1
}

Write-Host ("Updated {0} files" -f $updated)
