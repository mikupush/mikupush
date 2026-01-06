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

        try self.saveCopyToAppGroup(
            url: originalUrl,
            container: sharedContainerURL
        )

        return originalUrl
    }

    func saveCopyToAppGroup(url: URL, container: URL) throws {
        self.logger.info("saving file item to app group")
        let destinationUrl = container.appendingPathComponent(url.lastPathComponent)

        do {
            if FileManager.default.fileExists(atPath: destinationUrl.path) {
                try FileManager.default.removeItem(at: destinationUrl)
            }

            try FileManager.default.copyItem(at: url, to: destinationUrl)
            self.logger.info("File copied to: \(destinationUrl.path)")
        } catch {
            self.logger.error("Error copying file: \(error.localizedDescription)")
            throw error
        }
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
