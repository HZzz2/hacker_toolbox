use eframe::{
    egui::{self, RichText},
    epaint::FontId,
};

pub fn size30(text: &str) -> RichText {
    RichText::new(text).font(FontId::proportional(30.0))
}
pub fn size25(text: &str) -> RichText {
    RichText::new(text).font(FontId::proportional(25.0))
}

// pub fn font_size(text:&mut str,size:f32) -> RichText{
//     RichText::new(text).font(FontId::proportional(size))
// }

pub fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../../fonts/Aa潇洒瘦金书.ttf")),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}
