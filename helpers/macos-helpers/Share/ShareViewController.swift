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

import Cocoa
import Foundation
import UniformTypeIdentifiers
import os

class ShareViewController: NSViewController {
    private let logger = Logger(subsystem: Bundle.main.bundleIdentifier!, category: "share")
    private let dataQueue = DispatchQueue(label: "io.mikupush.dataQueue")

    override func loadView() {
        self.view = NSView(frame: .zero)
    }

    override func viewDidLoad() {
        super.viewDidLoad()

        let jsonEncoder = JSONEncoder()
        let groupIdentifier = "group.io.mikupush.client"
        
        guard let sharedContainerURL = FileManager.default.containerURL(forSecurityApplicationGroupIdentifier: groupIdentifier) else {
            self.logger.error("Not able to get into the app group.")
            return
        }
        
        guard let items = self.extensionContext?.inputItems as? [NSExtensionItem] else {
            self.closeExtension()
            return
        }
        
        self.logger.info("launching items url retrieve task")
 
        Task {
            let uuid = UUID().uuidString.lowercased()
            var fileUrls: [String] = []
            self.logger.info("extracting share items paths")
            
            for item in items {
                for attachment in item.attachments ?? [] {
                    self.logger.debug("checking item is url type")
                    if attachment.hasItemConformingToTypeIdentifier(UTType.fileURL.identifier) {
                        do {
                            if let url = try await self.retrieveSelectedFileUrl(
                                attachment: attachment,
                                sharedContainerURL: sharedContainerURL,
                                uuid: uuid
                            ) {
                                fileUrls.append(url.path())
                            }
                        } catch {
                            self.logger.warning("error retrieving file url: \(error)")
                        }
                    }
                }
            }

            self.finalizeAndSend(
                fileUrls: fileUrls,
                encoder: jsonEncoder,
                container: sharedContainerURL,
                uuid: uuid
            )
        }
    }
    
    func retrieveSelectedFileUrl(attachment: NSItemProvider, sharedContainerURL: URL, uuid: String) async throws -> URL? {
        self.logger.debug("retrieving item url")

        let item = try await attachment.loadItem(forTypeIdentifier: UTType.fileURL.identifier, options: nil)

        var resolvedUrl: URL? = nil

        if let nsUrl = item as? NSURL {
            resolvedUrl = nsUrl as URL
        }
        else if let url = item as? URL {
            resolvedUrl = url
        }
        else if let data = item as? Data {
            if let urlString = String(data: data, encoding: .utf8) {
                let cleanString = urlString.trimmingCharacters(in: .whitespacesAndNewlines)
                resolvedUrl = URL(string: cleanString) ?? URL(fileURLWithPath: cleanString)
            }
        }

        guard let originalUrl = resolvedUrl else {
            self.logger.error("url not resolved")
            return nil
        }

        return try self.copyFileToAppGroup(
            originalUrl: originalUrl,
            container: sharedContainerURL,
            uuid: uuid
        )
    }

    func finalizeAndSend(fileUrls: [String], encoder: JSONEncoder, container: URL, uuid: String) {
        if fileUrls.isEmpty {
            self.logger.warning("No files collected.")
            self.closeExtension()
            return
        }

        let fileName = "\(uuid).json"
        
        do {
            let json = try encoder.encode(fileUrls)
            try json.write(to: container.appendingPathComponent(fileName))
            self.logger.info("JSON written successfully with \(fileUrls.count) files")
        } catch {
            self.logger.error("error writing file: \(error)")
            self.closeExtension()
            return
        }

        if let url = URL(string: "mikupush:///share/\(fileName)") {
            NSWorkspace.shared.open(url)
        }

        self.closeExtension()
    }

    func copyFileToAppGroup(originalUrl: URL, container: URL, uuid: String) throws -> URL {
        let fileManager = FileManager.default

        let requestDir = container.appendingPathComponent(uuid, isDirectory: true)
        try fileManager.createDirectory(at: requestDir, withIntermediateDirectories: true)

        let fileName = originalUrl.lastPathComponent
        let destURL = requestDir.appendingPathComponent(fileName)

        self.logger.info("copying file to app group: \(originalUrl.path) -> \(destURL.path)")

        try fileManager.copyItem(at: originalUrl, to: destURL)
        return destURL
    }

    func closeExtension() {
        self.extensionContext?.completeRequest(returningItems: [], completionHandler: nil)
    }
}