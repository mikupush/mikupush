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
            var fileUrls: [String] = []
            self.logger.info("extracting share items paths")
            
            for item in items {
                for attachment in item.attachments ?? [] {
                    self.logger.debug("checking item is url type")
                    if attachment.hasItemConformingToTypeIdentifier(UTType.fileURL.identifier) {
                        do {
                            if let url = try await self.retrieveSelectedFileUrl(
                                attachment: attachment,
                                sharedContainerURL: sharedContainerURL
                            ) {
                                fileUrls.append(url.path())
                            }
                        } catch {
                            self.logger.warning("error retrieving file url: \(error)")
                        }
                    }
                }
            }

            self.finalizeAndSend(fileUrls: fileUrls, encoder: jsonEncoder, container: sharedContainerURL)
        }
    }
    
    func retrieveSelectedFileUrl(attachment: NSItemProvider, sharedContainerURL: URL) async throws -> URL? {
        self.logger.debug("retrieving item url")

        let item = try await attachment.loadItem(forTypeIdentifier: UTType.fileURL.identifier, options: nil)
        
        self.logger.info("item type: \(type(of: item), privacy: .public)")
        self.logger.info("item content: \(String(describing: item), privacy: .public)")
        
        var resolvedUrl: URL? = nil
        
        if let nsUrl = item as? NSURL {
            resolvedUrl = nsUrl as URL
        }
        else if let url = item as? URL {
            resolvedUrl = url
        }
        else if let data = item as? Data {
            self.logger.info("Item is Data. Trying to convert to String...")

            if let urlString = String(data: data, encoding: .utf8) {
                let cleanString = urlString.trimmingCharacters(in: .whitespacesAndNewlines)
                self.logger.debug("data decoded string: \(cleanString, privacy: .public)")

                // Intentamos crear la URL desde el string
                if let url = URL(string: cleanString) {
                    resolvedUrl = url
                } else {
                    resolvedUrl = URL(fileURLWithPath: cleanString)
                }
            }
        }
            
        guard let originalUrl = resolvedUrl else {
            self.logger.error("url not resolved from item: \(String(describing: item), privacy: .public)")
            return nil
        }

        return originalUrl
    }

    func finalizeAndSend(fileUrls: [String], encoder: JSONEncoder, container: URL) {
        if fileUrls.isEmpty {
            self.logger.warning("No files collected.")
            self.closeExtension()
            return
        }

        let uuid = UUID().uuidString.lowercased()
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

    func closeExtension() {
        self.extensionContext?.completeRequest(returningItems: [], completionHandler: nil)
    }
}