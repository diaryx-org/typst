//! A stand-in for `pdf.rs` without the `pdf-images` feature: no PDF page is
//! ever loaded as an image, so the type is uninhabited, and `ImageKind::Pdf`
//! and the code that matches on it stay as they are. Added by diaryx-org; see
//! PATCH.md.

/// A specific page of a PDF acting as an image. Never made in this build.
#[derive(Clone, Hash)]
pub enum PdfImage {}

impl PdfImage {
    /// Returns the width of the image.
    pub fn width(&self) -> f32 {
        match *self {}
    }

    /// Returns the height of the image.
    pub fn height(&self) -> f32 {
        match *self {}
    }

    /// Returns the page index of the image.
    pub fn page_index(&self) -> usize {
        match *self {}
    }
}
