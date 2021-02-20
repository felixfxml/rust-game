extern crate fps_counter;
extern crate opengl_graphics;
extern crate piston_window;
extern crate piston;

use fps_counter::FPSCounter;
use opengl_graphics::GlGraphics;
use piston_window::*;
use piston_window::glyph_cache::rusttype::GlyphCache;
use piston_window::math::scale;
use std::fmt::{Display, Formatter};

struct Player {
    x: f64,
    y: f64,
    direction: Direction,
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Direction::Up => { write!(f, "Up") }
            Direction::Right => { write!(f, "Right") }
            Direction::Down => { write!(f, "Down") }
            Direction::Left => { write!(f, "Left") }
        }
    }
}

impl Player {
    fn move_player(&mut self) {
        match self.direction {
            Direction::Up => {
                self.y -= 25.0;
                for x in "test".bytes() {
                    println!("bytes: {}", x as char);
                };
            }
            Direction::Right => {
                self.x += 25.0;
            }
            Direction::Down => {
                self.y += 25.0;
            }
            Direction::Left => {
                self.x -= 25.0;
            }
        }
    }
}

fn main() {
    let mut fps = FPSCounter::new();

    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("Test", [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .vsync(false)
        //.samples(4)
        .build().unwrap();

    window.set_ups(20);
    window.set_max_fps(300);

    let mut gl = GlGraphics::new(opengl);

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let glyphs =
        &mut GlyphCache::new("resources/fonts/segoeui.ttf",
                             (),
                             texture_settings)
            .unwrap();

    let mut player = Player {
        x: 0.0,
        y: 0.0,
        direction: Direction::Up,
    };

    while let Some(e) = window.next() {
        match e {
            Event::Loop(event) => {
                match event {
                    Loop::Update(_args) => {}
                    Loop::Render(render_args) => {
                        let frames = fps.tick();
                        //println!("fps: {}", frames);

                        gl.draw(render_args.viewport(), |ctx, g| {
                            clear([1.0; 4], g);

                            scale(0.2, 0.2);

                            let rect: Rectangle = Rectangle::new([0.2, 0.3, 0.4, 1.0]);
                            rect.draw([player.x, player.y, 50.0, 50.0],
                                      &ctx.draw_state,
                                      ctx.transform,
                                      g);

                            let mut text = Text::new(200);
                            text.color = [0.0, 0.0, 0.0, 1.0];
                            let mut s = "FPS: ".to_string();
                            s.push_str(&frames.to_string());
                            text.draw(&s,
                                      glyphs,
                                      &ctx.draw_state,
                                      ctx.trans(0.0, 200.0).transform,
                                      g).unwrap();
                        });
                    }
                    _ => {}
                }
            }
            Event::Input(input, _) => {
                match input {
                    Input::Button(args) => {
                        match args.state {
                            ButtonState::Press => {
                                match args.button {
                                    Button::Keyboard(key) => {
                                        match key {
                                            Key::W => {
                                                player.direction = Direction::Up;
                                                player.move_player();
                                            }
                                            Key::A => {
                                                player.direction = Direction::Left;
                                                player.move_player();
                                            }
                                            Key::S => {
                                                player.direction = Direction::Down;
                                                player.move_player();
                                            }
                                            Key::D => {
                                                player.direction = Direction::Right;
                                                player.move_player();
                                            }
                                            _ => {}
                                        }
                                    }
                                    Button::Mouse(button) => {
                                        match button {
                                            MouseButton::Left => {
                                                player.x = 0.0;
                                                player.y = 0.0;
                                            }
                                            _ => {}
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            ButtonState::Release => {
                                match args.button {
                                    Button::Keyboard(_) => {}
                                    Button::Mouse(_) => {}
                                    _ => {}
                                }
                            }
                        }
                    }
                    Input::Move(_) => {}
                    Input::Text(_) => {}
                    Input::Resize(_) => {}
                    Input::Focus(_) => {}
                    Input::Cursor(_) => {}
                    Input::FileDrag(_) => {}
                    Input::Close(_) => {}
                }
            }
            _ => {}
        }
    }
}