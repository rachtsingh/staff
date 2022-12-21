use crate::{
    duration::{Duration, DurationKind},
    render::Renderer,
};
use svg::Node;
use text_svg::Glyph;

pub struct NoteHead {
    pub index: i64,
    pub x: f64,
}

impl NoteHead {
    pub fn new(index: i64, x: f64) -> Self {
        Self { index, x }
    }

    pub fn draw(
        &self,
        note_x: f64,
        top: f64,
        duration: Duration,
        renderer: &Renderer,
        node: &mut impl Node,
    ) {
        let c = match duration.kind {
            DurationKind::Quarter => '𝅘',
            DurationKind::Half => '𝅗',
            DurationKind::Whole => '𝅝',
        };
        let glyph = Glyph::new(&renderer.font, c, 75.);
        let dot_glyph = if duration.is_dotted {
            Some(Glyph::new(&renderer.font, '.', 75.))
        } else {
            None
        };

        self.draw_with_glyph(note_x, top, &glyph, dot_glyph.as_ref(), renderer, node)
    }

    pub fn draw_with_glyph(
        &self,
        note_x: f64,
        top: f64,
        glyph: &Glyph,
        dot_glyph: Option<&Glyph>,
        renderer: &Renderer,
        node: &mut impl Node,
    ) {
        if let Some(dot_glyph) = dot_glyph {
            node.append(dot_glyph.path(
                (note_x + self.x + renderer.note_rx * 1.5 + renderer.stroke_width) as _,
                (top + renderer.note_ry * (self.index as f64 - 1.)) as _,
            ));
        }

        node.append(glyph.path(
            (note_x + self.x) as _,
            (top + renderer.note_ry * (self.index as f64 - 1.)) as _,
        ));
    }
}
