#![allow(dead_code)]

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod lib;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use lib::grid::*;
use lib::utils::*;
use lib::creatures::*;

pub struct Environment {
    gl: GlGraphics,
}

impl Environment {
    fn new(opengl: OpenGL) -> Self {
        Environment {
            gl: GlGraphics::new(opengl),
        }
    }

    fn render(&mut self, args: &RenderArgs, grid: &mut Grid) {
        use graphics::*;

        const WHITE: Color = [1.0, 1.0, 1.0, 1.0];

        let creature_width: f64 = args.width as f64 / grid.width as f64;
        let creature_height: f64 = args.height as f64/ grid.height as f64;
        
        let square = rectangle::square(0.0, 0.0, creature_width);
        self.gl.draw(args.viewport(), |c, gl|{

            for (i, creature) in grid.iter().enumerate() {
                let (x, y) = grid.index_to_position(i);
                let transform = c.transform.trans(x as f64 * creature_width, y as f64 * creature_height);
                rectangle(creature.color, square, transform, gl);
            }

        });
    }

    fn update(&mut self, args: &UpdateArgs, grid: &mut Grid) {
        for (i, creature) in grid.iter().enumerate() {
            let position = grid.index_to_position(i);
            creature.act(position, &mut grid);
        }
    }
}

fn main() {
    const WINDOW_WIDTH: u32 = 600;
    const WINDOW_HEIGHT: u32 = 600;

    const GRID_WIDTH: u32 = WINDOW_WIDTH / 3;
    const GRID_HEIGHT: u32 = WINDOW_HEIGHT / 3;

    let opengl = OpenGL::V3_2;
    let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);

    let mut window: Window = WindowSettings::new(
            "My Little Habitat",
            [WINDOW_WIDTH, WINDOW_HEIGHT]
        ).opengl(opengl)
         .exit_on_esc(true)
         .build()
         .unwrap();
    
    let mut env = Environment::new(opengl);
    let mut events = Events::new(EventSettings::new());

    let width_scale = (WINDOW_WIDTH as u32 / GRID_WIDTH) as f64;
    let height_scale = (WINDOW_HEIGHT as u32 / GRID_HEIGHT) as f64;

    let mut position: Position = (0, 0);

    while let Some(e) = events.next(&mut window) {
        match e {
            Input::Render(render_args) => {
                env.render(&render_args, &mut grid);
            },
            Input::Update(update_args) => {
                env.update(&update_args, &mut grid);
            },
            Input::Release(button) => {
                if button == Button::Mouse(MouseButton::Left) {
                    grid.set_cell(position, get_creature(CreatureType::Plant));
                }
            },
            Input::Move(motion) => {
                if let Motion::MouseCursor(x, y) = motion {
                    let grid_x = (x  / width_scale) as u32;
                    let grid_y = (y  /  height_scale) as u32;
                    position = (grid_x, grid_y);
                }
            },
            _ => {}
        }
    }
}