use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use rgframework::backend::Graphics;

pub type GlBackend = (<GlGraphics as Graphics>::Texture, GlyphCache<'static>);
