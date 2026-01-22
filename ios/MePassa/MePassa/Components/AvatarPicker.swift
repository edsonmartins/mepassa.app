//
//  AvatarPicker.swift
//  MePassa
//
//  Created by MePassa Team
//  Copyright © 2026 MePassa. All rights reserved.
//

import SwiftUI
import PhotosUI

/// AvatarPicker - Pick avatar from camera or photo library
struct AvatarPickerSheet: View {
    @Environment(\.dismiss) var dismiss
    let onImageSelected: (UIImage) -> Void

    @State private var showImagePicker = false
    @State private var showCamera = false
    @State private var selectedItem: PhotosPickerItem?
    @State private var selectedImage: UIImage?

    var body: some View {
        NavigationView {
            List {
                Button(action: {
                    showCamera = true
                }) {
                    Label("Tirar foto", systemImage: "camera")
                }

                Button(action: {
                    showImagePicker = true
                }) {
                    Label("Escolher da galeria", systemImage: "photo")
                }
            }
            .navigationTitle("Foto de perfil")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button("Cancelar") {
                        dismiss()
                    }
                }
            }
        }
        .sheet(isPresented: $showCamera) {
            ImagePicker(sourceType: .camera) { image in
                onImageSelected(image)
                dismiss()
            }
        }
        .photosPicker(isPresented: $showImagePicker, selection: $selectedItem)
        .onChange(of: selectedItem) { newItem in
            Task {
                if let data = try? await newItem?.loadTransferable(type: Data.self),
                   let image = UIImage(data: data) {
                    onImageSelected(image)
                    dismiss()
                }
            }
        }
    }
}

/// ImagePicker - UIImagePickerController wrapper
struct ImagePicker: UIViewControllerRepresentable {
    let sourceType: UIImagePickerController.SourceType
    let onImagePicked: (UIImage) -> Void

    func makeUIViewController(context: Context) -> UIImagePickerController {
        let picker = UIImagePickerController()
        picker.sourceType = sourceType
        picker.delegate = context.coordinator
        return picker
    }

    func updateUIViewController(_ uiViewController: UIImagePickerController, context: Context) {}

    func makeCoordinator() -> Coordinator {
        Coordinator(self)
    }

    class Coordinator: NSObject, UIImagePickerControllerDelegate, UINavigationControllerDelegate {
        let parent: ImagePicker

        init(_ parent: ImagePicker) {
            self.parent = parent
        }

        func imagePickerController(_ picker: UIImagePickerController, didFinishPickingMediaWithInfo info: [UIImagePickerController.InfoKey : Any]) {
            if let image = info[.originalImage] as? UIImage {
                parent.onImagePicked(image)
            }
            picker.dismiss(animated: true)
        }

        func imagePickerControllerDidCancel(_ picker: UIImagePickerController) {
            picker.dismiss(animated: true)
        }
    }
}

#Preview {
    AvatarPickerSheet { _ in }
}
