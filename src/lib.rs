pub mod wrap;
pub use wrap::*;

pub trait PdfExtractorTrait {
    fn extract_pdf_text(args: ExtractPdfTextArgs) -> Result<String, String>;
}

pub struct PdfExtractorModule;

pub struct ExtractPdfTextArgs {
    pub path_to_pdf: String,
}

impl PdfExtractorTrait for PdfExtractorModule {
    fn extractText(args: ExtractPdfTextArgs) -> Result<String, String> {
        let path_to_pdf = args.path_to_pdf;

        let bytes = match std::fs::read(&path_to_pdf) {
            Ok(bytes) => bytes,
            Err(_) => return Err(format!("Failed to read the PDF file: {}", path_to_pdf)),
        };

        let extractor = match pdf_extract::TextExtractor::from_bytes(&bytes, pdf_extract::FileFormat::Pdf) {
            Ok(extractor) => extractor,
            Err(_) => return Err(String::from("Failed to create TextExtractor from bytes")),
        };

        let text = match extractor.extract_text() {
            Ok(text) => text,
            Err(_) => return Err(String::from("Failed to extract text from PDF")),
        };

        Ok(text)
    }
}
