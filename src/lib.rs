pub extern crate gfx_graphics;
pub extern crate graphics;
pub extern crate rusttype;

pub mod gfx_prelude {
    use gfx_device_gl::Resources;
    use gfx_graphics::{GfxGraphics, GlyphCache};

    pub use graphics::Graphics;

    pub use gfx_graphics::TextureSettings;

    pub type Window = glutin_window::GlutinWindow;

    pub type Device = gfx_device_gl::Device;

    pub type Factory = gfx_device_gl::Factory;
    pub type CommandBuffer = gfx_device_gl::CommandBuffer;
    pub type Context = graphics::Context;

    pub type Encoder = gfx::Encoder<Resources, CommandBuffer>;

    pub type Gfx2d = gfx_graphics::Gfx2d<Resources>;

    pub type G2d<'a> = GfxGraphics<'a, Resources, CommandBuffer>;
    pub type Glyphs<'a> = GlyphCache<'a, Factory, Resources>;

    pub type DepthStencilView = gfx::handle::DepthStencilView<Resources, gfx::format::DepthStencil>;
    pub type RenderTargetView = gfx::handle::RenderTargetView<Resources, gfx::format::Srgba8>;
}

#[macro_use]
pub mod resources;

pub use self::resources::{Resource, Resources};

pub mod draw;
mod state_machine;
pub use self::state_machine::{State, StateMachine, StateTransition};
pub use crate::gfx_prelude::{Context, G2d};
pub use piston::{event_loop, input, window};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
