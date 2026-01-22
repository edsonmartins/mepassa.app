//
//  MediaPickerViewModel.swift
//  MePassa
//
//  ViewModel for managing media selection and upload
//

import SwiftUI
import Combine

/// Upload state
enum UploadState: Equatable {
    case idle
    case uploading(current: Int, total: Int)
    case success
    case error(String)
}

/// Media picker view model
class MediaPickerViewModel: ObservableObject {
    @Published var selectedImages: [UIImage] = []
    @Published var uploadState: UploadState = .idle

    private let mePassaCore: MePassaCore

    init(mePassaCore: MePassaCore = .shared) {
        self.mePassaCore = mePassaCore
    }

    /// Add images to selection
    func addImages(_ images: [UIImage]) {
        selectedImages.append(contentsOf: images)
    }

    /// Remove image at index
    func removeImage(at index: Int) {
        guard index >= 0 && index < selectedImages.count else { return }
        selectedImages.remove(at: index)
    }

    /// Clear all selected images
    func clearSelection() {
        selectedImages.removeAll()
    }

    /// Upload images to conversation
    func uploadImages(to conversationId: String) {
        guard !selectedImages.isEmpty else { return }

        uploadState = .uploading(current: 0, total: selectedImages.count)

        Task {
            do {
                for (index, image) in selectedImages.enumerated() {
                    try await uploadSingleImage(image, to: conversationId)

                    await MainActor.run {
                        uploadState = .uploading(current: index + 1, total: selectedImages.count)
                    }
                }

                await MainActor.run {
                    uploadState = .success
                    clearSelection()
                }
            } catch {
                await MainActor.run {
                    uploadState = .error(error.localizedDescription)
                }
            }
        }
    }

    /// Upload a single image with compression
    private func uploadSingleImage(_ image: UIImage, to conversationId: String) async throws {
        // Convert UIImage to JPEG data
        guard let imageData = image.jpegData(compressionQuality: 0.8) else {
            throw MediaError.compressionFailed
        }

        // TODO: Call FFI method to send media message
        // For now, just simulate delay
        try await Task.sleep(nanoseconds: 500_000_000) // 0.5s

        // In the future:
        // try await mePassaCore.sendMediaMessage(
        //     conversationId: conversationId,
        //     mediaType: "image",
        //     data: [UInt8](imageData),
        //     fileName: "image_\(Date().timeIntervalSince1970).jpg"
        // )
    }

    /// Reset upload state
    func resetUploadState() {
        uploadState = .idle
    }

    /// Get compressed JPEG data from UIImage
    func getCompressedJPEG(from image: UIImage, quality: CGFloat = 0.8) -> Data? {
        return image.jpegData(compressionQuality: quality)
    }

    /// Get thumbnail from UIImage
    func getThumbnail(from image: UIImage, size: CGSize = CGSize(width: 200, height: 200)) -> UIImage? {
        let renderer = UIGraphicsImageRenderer(size: size)
        return renderer.image { _ in
            image.draw(in: CGRect(origin: .zero, size: size))
        }
    }
}

/// Media-related errors
enum MediaError: LocalizedError {
    case compressionFailed
    case uploadFailed
    case invalidImage

    var errorDescription: String? {
        switch self {
        case .compressionFailed:
            return "Failed to compress image"
        case .uploadFailed:
            return "Failed to upload image"
        case .invalidImage:
            return "Invalid image format"
        }
    }
}

/// Media item metadata
struct MediaItem: Identifiable {
    let id = UUID()
    let image: UIImage
    let fileName: String?
    let fileSize: Int64?
}
