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
        // ----- 1. Determine page dimensions based on paper orientation -----
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

        // ----- 2. Define Font Size and Ideal Grid Pitch -----
        let font_size_half_pt: usize = 21;
        let char_size_twips: u32 = 210; // 10.5pt
        let ideal_line_pitch: u32 = 360; // 18pt (Comfortable line spacing)

        let min_margin: u32 = 567; // 10mm

        // ----- 3. Calculate Required Dimensions -----
        let (mut req_w, mut req_h, is_vertical) = if settings.orientation == "vertical" {
            (
                ideal_line_pitch * settings.lines_per_page,
                char_size_twips * settings.chars_per_line,
                true,
            )
        } else {
            (
                char_size_twips * settings.chars_per_line,
                ideal_line_pitch * settings.lines_per_page,
                false,
            )
        };

        // ----- 4. Clamp to Max Available Area -----
        let max_w = page_w.saturating_sub(min_margin * 2);
        let max_h = page_h.saturating_sub(min_margin * 2);

        if req_w > max_w {
            req_w = max_w;
        }
        if req_h > max_h {
            req_h = max_h;
        }

        // ----- 5. Calculate Actual Pitch based on Clamped Dimensions -----
        let (actual_line_pitch, actual_char_space) = if is_vertical {
            (
                req_w / settings.lines_per_page,
                req_h / settings.chars_per_line,
            )
        } else {
            (
                req_h / settings.lines_per_page,
                req_w / settings.chars_per_line,
            )
        };

        // ----- 6. Calculate Margins with Buffer -----
        // Add a tiny buffer (40 twips) to the available area to ensure Word's grid math doesn't drop the last line
        let buffer = 40;
        let margin_left = (page_w.saturating_sub(req_w + buffer) / 2).max(min_margin);
        let margin_right = page_w.saturating_sub(req_w + margin_left);
        let margin_top = (page_h.saturating_sub(req_h + buffer) / 2).max(min_margin);
        let margin_bottom = page_h.saturating_sub(req_h + margin_top);

        // ----- 7. Swap Margins for Word's Landscape XML Rotation -----
        let is_landscape = settings.paper_orientation == "landscape";
        let xml_margin_top = if is_landscape { margin_right } else { margin_top };
        let xml_margin_bottom = if is_landscape { margin_left } else { margin_bottom };
        let xml_margin_left = if is_landscape { margin_top } else { margin_left };
        let xml_margin_right = if is_landscape { margin_bottom } else { margin_right };

        // ----- 8. Build DocGrid and PageMargin -----
        let doc_grid = DocGrid::new()
            .grid_type(DocGridType::LinesAndChars)
            .line_pitch(actual_line_pitch as usize)
            .char_space(actual_char_space as isize);

        let page_margin = PageMargin::new()
            .top(xml_margin_top as i32)
            .bottom(xml_margin_bottom as i32)
            .left(xml_margin_left as i32)
            .right(xml_margin_right as i32)
            .header(min_margin as i32)
            .footer(min_margin as i32);

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
