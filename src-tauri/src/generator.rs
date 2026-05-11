use crate::layout::PageLayout;
use crate::settings::LayoutSettings;
use docx_rs::*;

pub fn create_docx(content: &str, settings: &LayoutSettings) -> Docx {
    let layout = PageLayout::new(settings);

    let mut section = Section::new()
        .page_size(layout.page_size)
        .page_orient(layout.page_orient)
        .page_margin(layout.page_margin)
        .doc_grid(layout.doc_grid);

    if settings.orientation == "vertical" {
        section = section.text_direction("tbRl".to_string());
    }

    if settings.nombre {
        let align = if settings.nombre_position == "left" {
            AlignmentType::Left
        } else {
            AlignmentType::Center
        };

        let footer = Footer::new().add_paragraph(
            Paragraph::new()
                .align(align)
                .add_run(Run::new().add_text("- ").size(layout.font_size_half_pt))
                .add_page_num(PageNum::new())
                .add_run(Run::new().add_text(" -").size(layout.font_size_half_pt)),
        );
        section = section.footer(footer);
    }

    let line_spacing_twips = layout.char_size_twips as i32;

    for line in content.lines() {
        let text = if line.is_empty() { "\u{3000}" } else { line }; // full-width space for blank lines

        let p = Paragraph::new()
            .align(AlignmentType::Both)
            .line_spacing(LineSpacing::new().line(line_spacing_twips))
            .add_run(Run::new().add_text(text).size(layout.font_size_half_pt));

        section = section.add_paragraph(p);
    }

    Docx::new().add_section(section)
}
