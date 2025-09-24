#!/usr/bin/env python3

import glob
import re

license_header_rust = """
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
"""

license_header_typescript = """
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
"""

print("adding license header to source code")

rust_source_files = glob.glob('src-tauri/**/*.rs', recursive=True) \
                    + glob.glob('lib/**/*.rs', recursive=True)

typescript_source_files = glob.glob('src/**/*.ts', recursive=True) \
                        + glob.glob('src/**/*.tsx', recursive=True) \
                        + glob.glob('src/**/*.css', recursive=True)

rust_regex = re.compile(r"^/// Copyright 2025 Miku Push! Team")
typescript_regex = re.compile(r"^/\*\*\n \* Copyright 2025 Miku Push! Team")

exclude_files = [
    "src/vite-env.d.ts",
    "src-tauri/build.rs",
    "src-tauri/src/main.rs"
]

def add_license(source_files: list[str], license_header_content: str, regex: re.Pattern[str]):
    for source_file in source_files:
        with open(source_file, "r", encoding="utf-8") as file:
            if source_file in exclude_files:
                print(f"excluding file {source_file}")
                continue

            source_code = file.read().strip()

            if regex.match(source_code):
                print(f"file {source_file} already has license header")
                continue

        with open(source_file, "w", encoding="utf-8") as file:
            file.write(license_header_content.strip() + "\n\n" + source_code)


add_license(rust_source_files, license_header_rust, rust_regex)
add_license(typescript_source_files, license_header_typescript, typescript_regex)
print("added license header to all source code")
