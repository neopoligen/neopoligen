use resvg;
use std::path::PathBuf;
use tiny_skia::Pixmap;
use usvg::fontdb::Database;
use usvg::Options;
use usvg::Tree;

pub struct OgImage {
    pub text_areas: Vec<OgImageTextArea>,
}

impl OgImage {
    pub fn text_areas(&self) -> String {
        self.text_areas
            .iter()
            .map(|ta| ta.svg_elements())
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn render_svg(&self, output_path: &PathBuf) {
        let source = format!(
            r#"
<svg version="1.1"
     width="1200" height="630"
     xmlns="http://www.w3.org/2000/svg">
    <rect width="100%" height="100%" fill="black" />
    {}
</svg>
"#,
            self.text_areas()
        );
        let opts = Options::default();
        let mut fonts = Database::new();
        fonts.load_system_fonts();
        let tree = Tree::from_str(&source, &opts, &fonts).unwrap();
        let size = tree.size();
        let mut pixmap = Pixmap::new(size.width() as u32, size.height() as u32).unwrap();
        resvg::render(
            &tree,
            tiny_skia::Transform::identity(),
            &mut pixmap.as_mut(),
        );
        pixmap.save_png(output_path).expect("Failed to save PNG");
    }
}

pub struct OgImageTextArea {
    pub color: String,
    pub font_family: String,
    pub font_size: usize,
    pub line_height: usize,
    pub max_char_width: usize,
    pub max_lines: usize,
    pub text: String,
    pub x: usize,
    pub y: usize,
}

impl OgImageTextArea {
    pub fn svg_elements(&self) -> String {
        let mut lines: Vec<String> = vec![];
        let words = self.text.split(" ").collect::<Vec<&str>>();
        let _ = words.iter().fold(0, |acc, w| {
            let position = acc + w.len();
            let line_index = position / self.max_char_width;
            if lines.len() == line_index {
                lines.push("".to_string());
            }
            if line_index == self.max_lines - 1 {
                if lines[line_index].len() > self.max_char_width - 8 {
                    if lines[line_index].len() < self.max_char_width {
                        // pad space to make sure it's the last word added
                        lines[line_index].push_str("...                      ");
                    }
                } else {
                    lines[line_index].push_str(w);
                }
            } else {
                lines[line_index].push_str(w);
            }
            lines[line_index].push_str(" ");
            position
        });
        lines
            .iter()
            .enumerate()
            .take(self.max_lines)
            .map(|(indx, value)| {
                format!(
                    r#"<text x="{}" y="{}" style="font-family: {}; font-size: {}; fill: {};">{}</text>"#,
                    self.x,
                    (self.line_height * indx) + self.y,
                    self.font_family,
                    self.font_size,
                    self.color,
                    value
                )
            })
            .collect::<Vec<String>>()
            .join("")
    }
}
