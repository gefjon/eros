use crate::gfx_prelude::*;
use piston::input::*;
use crate::resources::Resources;

pub trait State {
    fn init(&mut self, _resources: &Resources) {}
    fn draw(&mut self, ctx: Context, gfx: &mut G2d, args: RenderArgs, resources: &Resources);
    fn handle_input(&mut self, input: Input) -> StateTransition {
        match input {
            Input::Button(args) => self.handle_button_event(args),
            _ => StateTransition::Continue,
        }
    }
    fn update(&mut self, _update_args: UpdateArgs) -> StateTransition {
        StateTransition::Continue
    }
    fn idle(&mut self, _idle_args: IdleArgs, _resources: &Resources) -> StateTransition {
        StateTransition::Continue
    }
    fn handle_button_event(&mut self, args: ButtonArgs) -> StateTransition {
        match args {
            ButtonArgs {
                state: ButtonState::Press,
                button,
                ..
            } => self.handle_button_pressed(button),
            ButtonArgs {
                state: ButtonState::Release,
                button,
                ..
            } => self.handle_button_released(button),
        }
    }
    fn handle_button_pressed(&mut self, button: Button) -> StateTransition {
        match button {
            Button::Keyboard(k) => self.handle_key_pressed(k),
            _ => StateTransition::Continue,
        }
    }
    fn handle_button_released(&mut self, button: Button) -> StateTransition {
        match button {
            Button::Keyboard(k) => self.handle_key_released(k),
            _ => StateTransition::Continue,
        }
    }
    fn handle_key_pressed(&mut self, _key: Key) -> StateTransition {
        StateTransition::Continue
    }
    fn handle_key_released(&mut self, _key: Key) -> StateTransition {
        StateTransition::Continue
    }
}

pub enum StateTransition {
    Quit,
    Continue,
    Replace(Box<dyn State>),
    Return,
    Push(Box<dyn State>),
}
