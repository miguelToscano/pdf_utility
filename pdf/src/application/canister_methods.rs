use candid::{CandidType, Principal};
use ic_kit::candid::{candid_method, export_service};
use ic_kit::ic;
use ic_kit::macros::*;
use ic_kit::*;
use serde::Deserialize;
// use genpdf;
// use printpdf;

// pub const LIBERATION_SANS_REGULAR: &[u8] = include_bytes!("../../../fonts/LiberationSans/LiberationSans-Regular.ttf");
// pub const LIBERATION_SANS_BOLD: &[u8] = include_bytes!("../../../fonts/LiberationSans/LiberationSans-Bold.ttf");
// pub const LIBERATION_SANS_ITALIC: &[u8] = include_bytes!("../../../fonts/LiberationSans/LiberationSans-Italic.ttf");
// pub const LIBERATION_SANS_BOLD_ITALIC: &[u8] = include_bytes!("../../../fonts/LiberationSans/LiberationSans-BoldItalic.ttf");

#[query]
#[candid_method(query)]
pub fn name() -> String {
    String::from("W3NS")
}

// pub fn generate_font_family() -> Result<genpdf::fonts::FontFamily<genpdf::fonts::FontData>, genpdf::error::Error> {
//     Ok(genpdf::fonts::FontFamily {
//         regular: genpdf::fonts::FontData::new(LIBERATION_SANS_REGULAR.to_vec(), None)?,
//         bold: genpdf::fonts::FontData::new(LIBERATION_SANS_BOLD.to_vec(), None)?,
//         italic: genpdf::fonts::FontData::new(LIBERATION_SANS_ITALIC.to_vec(), None)?,
//         bold_italic: genpdf::fonts::FontData::new(LIBERATION_SANS_BOLD_ITALIC.to_vec(), None)?,
//     })
// }

// #[update]
// #[candid_method(update)]
// pub async fn generate_pdf() -> Result<(), ()> { 
//     // Load a font from the file system
//     // let font_family = genpdf::fonts::from_files("../../fonts", "LiberationSans", None)
//     // .expect("Failed to load font family");
//     let font_family = generate_font_family().expect("Failed to load font family");
//     // genpdf::fonts::FontData::new(LIBERATION_SANS_FONT, None);
//     // Create a document and set the default font family
//     let mut doc = genpdf::Document::new(font_family);
//     // Change the default settings
//     doc.set_title("Demo document");
//     // Customize the pages
//     let mut decorator = genpdf::SimplePageDecorator::new();
//     decorator.set_margins(10);
//     doc.set_page_decorator(decorator);
//     // Add one or more elements
//     doc.push(genpdf::elements::Paragraph::new("This is a demo document."));
//     // Render the document and write it to a file
//     doc.render_to_file("output.pdf").expect("Failed to write PDF file");
//     Ok(())
// }

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        println!("{:?}", dir);
        let dir = dir.parent().unwrap().join("candid");
        write(dir.join("pdf.did"), export_candid()).expect("Write failed.");
    }
}