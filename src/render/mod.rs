//! Sheet music engraving
use rusttype::Font;
use svg::{
    node::element::{Line, Rectangle},
    Document, Node,
};

pub mod measure;
pub use measure::{Clef, Measure};

mod note;
pub use note::Note;

pub trait Draw {
    fn draw(&self, x: f64, y: f64, renderer: &Renderer, node: &mut impl Node);
}

pub struct Renderer {
    pub document_padding: f64,
    pub note_rx: f64,
    pub note_ry: f64,
    pub padding: f64,
    pub stroke_width: f64,
    pub accidental_size: f64,
    pub width: f64,
    pub height: f64,
    pub font: Font<'static>,
    pub min_spacing: f64,
}

impl Default for Renderer {
    fn default() -> Self {
        let font_data = include_bytes!("../../NotoMusic.ttf");
        let font = Font::try_from_bytes(font_data as &[u8]).expect("Error loading font Noto Music from file.");

        Self {
            document_padding: 20.,
            note_rx: 10.,
            note_ry: 6.,
            padding: 10.,
            stroke_width: 1.5,
            accidental_size: 80.,
            width: 500.,
            height: 200.,
            font,
            min_spacing: 18.,
        }
    }
}

impl Renderer {
    pub fn render(&self, staff: &Staff) -> Document {
        let mut document = svg::Document::new()
            .set("width", self.width)
            .set("height", self.height);

        document.append(
            Rectangle::new()
                .set("fill", "#fff")
                .set("x", 0)
                .set("y", 0)
                .set("width", self.width)
                .set("height", self.height),
        );

        let mut y = 0.;
        for row in &staff.rows {
            let mut x = self.stroke_width + self.document_padding;

            let measures_width = row
                .measures
                .iter()
                .map(|measure| measure.width)
                .sum::<f64>();
            let remaining = self.width - measures_width - self.document_padding * 2.;
            let measure_exta = remaining / row.measures.len() as f64;

            for (index, measure) in row.measures.iter().enumerate() {
                x = measure.svg(x, y, measure_exta, index, self, &mut document);
            }

            y += 100.;
        }

        document
    }

    fn draw_line<T: Node>(&self, node: &mut T, x1: f64, y1: f64, x2: f64, y2: f64) {
        node.append(
            Line::new()
                .set("stroke", "#000")
                .set("stroke-width", self.stroke_width)
                .set("x1", x1)
                .set("y1", y1)
                .set("x2", x2)
                .set("y2", y2),
        )
    }
}

pub struct Row<'r> {
    measures: Vec<Measure<'r>>,
    width: f64,
}

#[derive(Default)]
pub struct Staff<'r> {
    rows: Vec<Row<'r>>,
}

impl<'r> Staff<'r> {
    pub fn push(&mut self, renderer: &Renderer, measure: Measure<'r>) {
        if let Some(row) = self.rows.last_mut() {
            let width = row.width + measure.width;
            if width < renderer.width {
                row.measures.push(measure);
                row.width = width;
                return;
            }
        }

        let row = Row {
            width: measure.width,
            measures: vec![measure],
        };
        self.rows.push(row);
    }
}
