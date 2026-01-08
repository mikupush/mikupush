import fs from 'node:fs'
import path from 'node:path'
import process from 'node:process'
import { globSync } from 'glob'
import ignore from 'ignore'

const licenseHeader = `
// Miku Push! is a simple, lightweight, and open-source WeTransfer alternative for desktop.
// Copyright (C) 2025  Miku Push! Team
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
`

const licenseHeaderTypescript = `
/**
 * Miku Push! is a simple, lightweight, and open-source WeTransfer alternative for desktop.
 * Copyright (C) 2025  Miku Push! Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
`

const rustSourcePatterns = [
  'src-tauri/**/*.rs',
  'lib/**/*.rs'
]

const swiftSourcePatterns = [
  'helpers/macos-helpers/Share/*.swift',
]

const typescriptSourcePatterns = [
  'src/**/*.ts',
  'src/**/*.tsx',
  'src/**/*.css'
]

const excludePatterns = [
  'src/vite-env.d.ts',
  'src-tauri/build.rs',
  'src-tauri/src/main.rs',
  'src/components/ui/*.tsx',
  'lib/database/src/schema.rs'
]

const ignoredFiles = ignore().add(excludePatterns)
const regex = /^\/\/ Miku Push! is a simple, lightweight, and open-source WeTransfer alternative for desktop\./
const typescriptRegex = /^\/\*\*\n \* Miku Push! is a simple, lightweight, and open-source WeTransfer alternative for desktop\./

const rootDir = process.cwd()

function collectFiles(patterns: string[]): string[] {
  const files = new Set<string>()

  patterns.forEach((pattern) => {
    const matches = globSync(pattern, {
      cwd: rootDir,
      nodir: true
    })

    matches
      .filter(match => !ignoredFiles.ignores(match))
      .forEach((match) => files.add(match))
  })

  return Array.from(files).sort()
}

function getSourceCode(filePath: string): string {
  const absolutePath = path.resolve(rootDir, filePath)
  const content = fs.readFileSync(absolutePath, 'utf-8')
  return content.trim()
}

function addLicense(sourceFiles: string[], licenseHeader: string, regex: RegExp): void {
  const trimmedHeader = licenseHeader.trim()

  for (const sourceFile of sourceFiles) {
    const sourceCode = getSourceCode(sourceFile)

    if (regex.test(sourceCode)) {
      console.log(`file ${sourceFile} already has license header`)
      continue
    }

    const absolutePath = path.resolve(rootDir, sourceFile)
    fs.writeFileSync(absolutePath, `${trimmedHeader}\n\n${sourceCode}`, 'utf-8')
    console.log('license header added on ', sourceFile)
  }
}

function removeLicense(sourceFiles: string[], licenseHeader: string): void {
  const trimmedHeader = licenseHeader.trim()

  for (const sourceFile of sourceFiles) {
    const sourceCode = getSourceCode(sourceFile)
    const absolutePath = path.resolve(rootDir, sourceFile)
    const updatedSource = sourceCode.replace(trimmedHeader, '').trim()
    fs.writeFileSync(absolutePath, updatedSource, 'utf-8')
    console.log('license header removed on ', sourceFile)
  }
}

const rustSourceFiles = collectFiles(rustSourcePatterns)
const swiftSourceFiles = collectFiles(swiftSourcePatterns)
const typescriptSourceFiles = collectFiles(typescriptSourcePatterns)

const args = process.argv.slice(2)
const shouldRemove = args.includes('--remove')

if (shouldRemove) {
  console.log('removing license header from source code')
  removeLicense(rustSourceFiles, licenseHeader)
  removeLicense(swiftSourceFiles, licenseHeader)
  removeLicense(typescriptSourceFiles, licenseHeaderTypescript)
  console.log('removed license header from source code')
} else {
  console.log('adding license header to source code')
  addLicense(rustSourceFiles, licenseHeader, regex)
  addLicense(swiftSourceFiles, licenseHeader, regex)
  addLicense(typescriptSourceFiles, licenseHeaderTypescript, typescriptRegex)
  console.log('added license header to all source code')
}
