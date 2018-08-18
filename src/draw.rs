use crate::state_machine::State;

use crate::gfx_prelude::*;

crate const GL_V: glutin_window::OpenGL = glutin_window::OpenGL::V3_2;
crate const SAMPLES: u8 = 4;

crate struct Draw {
    encoder: Encoder,
    g2d: Gfx2d,
    output_color: RenderTargetView,
    output_stencil: DepthStencilView,
    device: Device,
}

impl Draw {
    fn create_color_and_stencil(window: &Window) -> (RenderTargetView, DepthStencilView) {
        use gfx::{
            format::{DepthStencil, Formatted, Srgba8},
            memory::Typed,
        };
        use piston::window::Window;

        let draw_size = window.draw_size();
        let aa = SAMPLES as gfx::texture::NumSamples;

        let dim = (
            draw_size.width as u16,
            draw_size.height as u16,
            1,
            aa.into(),
        );

        let color_format = <Srgba8 as Formatted>::get_format();
        let depth_format = <DepthStencil as Formatted>::get_format();

        let (output_color, output_stencil) =
            gfx_device_gl::create_main_targets_raw(dim, color_format.0, depth_format.0);

        let output_color = Typed::new(output_color);
        let output_stencil = Typed::new(output_stencil);

        (output_color, output_stencil)
    }

    crate fn new(window: &mut Window) -> (Self, Factory) {
        use piston::window::OpenGLWindow;

        let (device, mut factory) =
            gfx_device_gl::create(|s| window.get_proc_address(s) as *const std::os::raw::c_void);

        let g2d = Gfx2d::new(GL_V, &mut factory);

        let (output_color, output_stencil) = Self::create_color_and_stencil(&*window);

        let encoder = factory.create_command_buffer().into();

        (Draw {
            device,
            encoder,
            g2d,
            output_color,
            output_stencil,
        }, factory)
    }

    crate fn draw(&mut self, state: &mut dyn State, args: piston::input::RenderArgs, resources: &crate::resources::Resources) {
        let Draw {
            ref mut encoder,
            ref mut g2d,
            ref output_color,
            ref output_stencil,
            ref mut device,
            ..
        } = self;
        g2d.draw(
            encoder,
            output_color,
            output_stencil,
            args.viewport(),
            |c, g| state.draw(c, g, args, resources),
        );
        encoder.flush(device);
    }
}
