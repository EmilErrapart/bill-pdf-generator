use wkhtmltopdf::{PageSize, PdfApplication};

fn main() {
    let html = include_str!("templates/template.html");
    let pdf_app = PdfApplication::new().expect("Failed to init PDF application");
    let mut pdfout = pdf_app.builder()
        .page_size(PageSize::A4)
        .build_from_html(&html)
        .expect("failed to build pdf");

    pdfout.save("arve.pdf").expect("failed to save foo.pdf");
}