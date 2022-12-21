use crate::render::{Draw, Renderer};
use svg::Node;
use text_svg::Glyph;

pub struct Clef<'r> {
    pub glyph: Glyph<'r>,
}

impl<'r> Clef<'r> {
    pub fn new(renderer: &'r Renderer) -> (Self, f64) {
        let clef_glyph = Glyph::new(&renderer.font, '𝄞', (renderer.note_ry * 10.) as _);
        let width = clef_glyph.bounding_box.width() as f64 + renderer.padding;
        let me = Self { glyph: clef_glyph };
        (me, width)
    }
}

impl Draw for Clef<'_> {
    fn draw(&self, x: f64, y: f64, renderer: &Renderer, node: &mut impl Node) {
        node.append(self.glyph.path(x as _, (y - renderer.note_ry) as _));
    }
}
