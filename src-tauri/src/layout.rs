use crate::settings::LayoutSettings;
use docx_rs::{DocGrid, DocGridType, PageMargin, PageOrientationType, PageSize};

pub struct PageLayout {
    pub page_size: PageSize,
    pub page_orient: PageOrientationType,
    pub page_margin: PageMargin,
    pub doc_grid: DocGrid,
    pub char_size_twips: u32,
    pub font_size_half_pt: usize,
}

impl PageLayout {
    pub fn new(settings: &LayoutSettings) -> Self {
        // ----- Determine page dimensions based on paper orientation -----
        // A4 in twips: portrait = 11906 x 16838
        let (page_w, page_h) = if settings.paper_orientation == "landscape" {
            (16838u32, 11906u32)
        } else {
            (11906u32, 16838u32)
        };

        let page_orient = if settings.paper_orientation == "landscape" {
            PageOrientationType::Landscape
        } else {
            PageOrientationType::Portrait
        };

        // Font size: 10.5 pt  →  half-points = 21,  twips = 210
        let font_size_half_pt: usize = 21;
        let char_size_twips: u32 = 210;

        // ----- Calculate layout based on text direction -----
        // For vertical writing: chars=height, lines=width
        // For horizontal writing: chars=width, lines=height
        let (content_w, content_h) = if settings.orientation == "vertical" {
            (
                char_size_twips * settings.lines_per_page,
                char_size_twips * settings.chars_per_line,
            )
        } else {
            (
                char_size_twips * settings.chars_per_line,
                char_size_twips * settings.lines_per_page,
            )
        };

        // ----- Calculate margins with a small buffer -----
        let min_margin: i32 = 360;
        let buffer: i32 = 40;

        let h_spare = page_w as i32 - content_w as i32 - buffer;
        let margin_left = (h_spare / 2).max(min_margin);
        let margin_right = margin_left;

        let v_spare = page_h as i32 - content_h as i32 - buffer;
        let margin_top = (v_spare / 2).max(min_margin);
        let margin_bottom = margin_top;

        // ----- Swap margins for Word's Landscape rotation -----
        let is_landscape = settings.paper_orientation == "landscape";
        let xml_margin_top = if is_landscape { margin_right } else { margin_top };
        let xml_margin_bottom = if is_landscape { margin_left } else { margin_bottom };
        let xml_margin_left = if is_landscape { margin_top } else { margin_left };
        let xml_margin_right = if is_landscape { margin_bottom } else { margin_right };

        // ----- Build DocGrid and PageMargin -----
        let doc_grid = DocGrid::new()
            .grid_type(DocGridType::LinesAndChars)
            .line_pitch(char_size_twips as usize)
            .char_space(0);

        let page_margin = PageMargin::new()
            .top(xml_margin_top)
            .bottom(xml_margin_bottom)
            .left(xml_margin_left)
            .right(xml_margin_right)
            .header(min_margin)
            .footer(min_margin);

        Self {
            page_size: PageSize::new().width(page_w).height(page_h),
            page_orient,
            page_margin,
            doc_grid,
            char_size_twips,
            font_size_half_pt,
        }
    }
}
