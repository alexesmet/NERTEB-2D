use ggez;
use ggez::{event, graphics, Context, GameResult};
use ggez::event::{KeyCode, KeyMods};

use std::f32::consts::PI;

mod conf {
    pub const FPS: u32 = 60;
    pub const SCREEN_SIZE: (f32, f32) = (800.0, 600.0);
}

struct State {
    // Here we will find the map, player, etc.
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ggez::timer::check_update_time(ctx, conf::FPS) {
            // Here, items from State should be updated.
            // It's good when ech item (player, entities) have theri update() function.
        }
        return Ok(());
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.6, 0.6, 0.6, 0.6].into());

        // Here the rendering happens.
        // You can access State fields by `this.field`

        // Example of drawing a rectangle.
        let rectangle = graphics::Mesh::new_rectangle(      // create a __MESH__ 
                ctx,
                graphics::DrawMode::fill(),                 // is filled
                graphics::Rect::new_i32(10, 10, 20, 20),    // Mesh is a rectangle; coords of corners
                                                            // (zero-point is a center of rect:
                                                            //  x, y, width, height)
                [0.0, 0.0, 0.0, 1.0].into(),                // RGBA color
            )?;

        let draw_params = graphics::DrawParam::new()        // This describes the __POSITION__ of created mesh
                .dest([50.0, 60.0])                         // coordinates of center on screen
                .rotation(PI / 6.0)                           // rotation around the center of the mesh
                .scale([1.5, 2.0]);

        graphics::draw(ctx, &rectangle, draw_params)?;      // Draw the resulting mesh on the screen


        graphics::present(ctx)?;                            // Something that helps timer to work
        ggez::timer::yield_now();                           // ...nevermind
        return Ok(());
    }


    fn key_up_event( &mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        // There are also other kyeboard and mouse functions, see:
        // https://docs.rs/ggez/0.5.1/ggez/event/trait.EventHandler.html
        match keycode {
            KeyCode::W => println!("W was pressed..."),
            _ => (),
        }
    }
}

fn main() -> GameResult {
    // For drawing we are using 'ggez' __library__, not engine :)
    // Docummentation (use this instead of google for this lib):
    // https://docs.rs/ggez/0.5.1/ggez/
    let (ctx, events_loop) = &mut ggez::ContextBuilder::new("nerteb-2d", "...")
        .window_setup(ggez::conf::WindowSetup::default().title("NERTEB-2D"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(conf::SCREEN_SIZE.0, conf::SCREEN_SIZE.1))
        .build()?;

    // create the world (can be loaded from file or whatever)
    let mut state = State {  };

    return event::run(ctx, events_loop, &mut state);
}
