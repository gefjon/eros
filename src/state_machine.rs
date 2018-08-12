use crate::gfx_prelude::Window;
mod state_trait;
pub use self::state_trait::{State, StateTransition};
use crate::draw::Draw;
use piston::input::*;
use std::convert::AsMut;

pub struct StateMachine {
    window: Window,
    draw: Draw,
    stack: Vec<Box<dyn State>>,
}

impl StateMachine {
    fn make_events() -> piston::event_loop::Events {
        use piston::event_loop::*;

        Events::new(EventSettings::new())
    }
    pub fn run(&mut self, initial_state: Box<dyn State>) {
        self.push(initial_state);

        let events = &mut Self::make_events();

        while let Some(e) = events.next(&mut self.window) {
            match e {
                Event::Loop(Loop::Render(args)) => self.draw(args),
                Event::Loop(Loop::Update(args)) => {
                    if self.update_and_should_quit(args) {
                        return;
                    }
                }
                Event::Loop(Loop::Idle(args)) => {
                    if self.idle_and_should_quit(args) {
                        return;
                    }
                }
                Event::Input(input) => {
                    if self.handle_input_and_should_quit(input) {
                        return;
                    }
                }
                _ => (),
            }
        }
    }
    fn make_window(title: &str, dims: (u32, u32)) -> Window {
        use piston::window::WindowSettings;

        WindowSettings::new(title, dims)
            .opengl(crate::draw::GL_V)
            .samples(crate::draw::SAMPLES)
            .build()
            .unwrap()
    }
    pub fn new(title: &str, dims: (u32, u32)) -> Self {
        let mut window = Self::make_window(title, dims);

        let draw = Draw::new(&mut window);

        StateMachine {
            window,
            draw,
            stack: Vec::new(),
        }
    }
    crate fn draw(&mut self, args: piston::input::RenderArgs) {
        let StateMachine {
            ref mut draw,
            ref mut stack,
            ..
        } = self;
        draw.draw(stack.last_mut().unwrap().as_mut(), args);
    }

    fn is_stack_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn handle_state_transition_and_should_quit(&mut self, trans: StateTransition) -> bool {
        match trans {
            StateTransition::Quit => true,
            StateTransition::Continue => false,
            StateTransition::Replace(new) => {
                *self.current_state_mut() = new;
                true
            }
            StateTransition::Push(new) => {
                self.push(new);
                true
            }
            StateTransition::Return => {
                self.pop();
                self.is_stack_empty()
            }
        }
    }

    #[cfg_attr(feature = "cargo-clippy", allow(borrowed_box))]
    fn current_state_mut(&mut self) -> &mut Box<dyn State> {
        let idx = self.stack.len() - 1;
        &mut self.stack[idx]
    }
    fn push(&mut self, state: Box<dyn State>) {
        self.stack.push(state);
    }
    fn pop(&mut self) -> Option<Box<dyn State>> {
        self.stack.pop()
    }
    fn update_and_should_quit(&mut self, args: UpdateArgs) -> bool {
        let trans = self.current_state_mut().update(args);
        self.handle_state_transition_and_should_quit(trans)
    }
    fn idle_and_should_quit(&mut self, args: IdleArgs) -> bool {
        let trans = self.current_state_mut().idle(args);
        self.handle_state_transition_and_should_quit(trans)
    }
    fn handle_input_and_should_quit(&mut self, input: Input) -> bool {
        let trans = self.current_state_mut().handle_input(input);
        self.handle_state_transition_and_should_quit(trans)
    }
}
