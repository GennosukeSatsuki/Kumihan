use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct LayoutSettings {
    pub orientation: String,       // "vertical" or "horizontal"
    pub paper_orientation: String, // "portrait" or "landscape"
    pub nombre: bool,
    pub nombre_position: String,   // "center" or "left"
    pub chars_per_line: u32,
    pub lines_per_page: u32,
}
