use ggez;
use ggez::{event, graphics, Context, GameResult};
use ggez::event::{KeyCode, KeyMods};

use std::f32::consts::PI;
use ggez::mint::Point2;
use ggez::input::mouse::MouseButton;
use core::ops;
use std::rc::Rc;

mod conf {
    pub const FPS: u32 = 60;
    pub const SCREEN_SIZE: (f32, f32) = (800.0, 600.0);
}

#[derive(Clone, Copy)]
pub struct Xy {
    pub x : f64,
    pub y : f64
}

pub trait XyProvider {
    fn get_x(&self) -> f64;
    fn get_y(&self) -> f64;
}

impl From<Xy> for Point2<f32> {
	fn from(xy : Xy) -> Self {
		return Point2 { x : xy.x as f32, y: xy.y as f32};
	}
}

impl XyProvider for Xy {
    fn get_x(&self) -> f64 { return self.x; }
    fn get_y(&self) -> f64 { return self.y; }
}

impl XyProvider for f64 {
    fn get_x(&self) -> f64 { return *self; }
    fn get_y(&self) -> f64 { return *self; }
}

impl XyProvider for f32 {
    fn get_x(&self) -> f64 { return *self as f64; }
    fn get_y(&self) -> f64 { return *self as f64; }
}

impl XyProvider for i32 {
    fn get_x(&self) -> f64 { return *self as f64; }
    fn get_y(&self) -> f64 { return *self as f64; }
}

impl XyProvider for i64 {
    fn get_x(&self) -> f64 { return *self as f64; }
    fn get_y(&self) -> f64 { return *self as f64; }
}

pub fn Xy<T : XyProvider, R : XyProvider>(tx : T, ty : R) -> Xy {
	return Xy::new(tx, ty);
}

impl Xy {
    fn new<T : XyProvider, R : XyProvider>(tx : T, ty : R) -> Self {
        return Xy { x: tx.get_x(), y: ty.get_y() }
    }
	fn set<T : XyProvider, R : XyProvider>(&mut self, tx : T, ty : R) -> & Self {
		self.x = tx.get_x();
		self.y = ty.get_y();
		return self;
	}
    // ...
}

impl ops::Add<Xy> for Xy {
	type Output = Xy;
	fn add(self, xy: Xy) -> Self {
		return Xy(self.x + xy.x, self.y + xy.x);
	}
}
impl ops::Sub<Xy> for Xy {
	type Output = Xy;
	fn sub(self, xy: Xy) -> Self {
		return Xy(self.x - xy.x, self.y - xy.x);
	}
}
impl ops::AddAssign<Xy> for Xy {
	fn add_assign(&mut self, xy: Xy) {
		self.x += xy.x;
		self.y += xy.y;
	}
}

impl ops::MulAssign<f64> for Xy {
	fn mul_assign(&mut self, coefficient: f64) {
		self.x *= coefficient;
		self.y *= coefficient;
	}
}

struct Movable {
	coordinates : Xy,
	movement_vector : Xy,
	acceleration_vector : Xy,
	braking_coefficient : f64
}

impl Movable {
	fn move0(&mut self) -> & Self {
		self.movement_vector += self.acceleration_vector;
		self.movement_vector *= 1 as f64 - self.braking_coefficient;
		self.coordinates     += self.movement_vector;
		return self;
	}
}

struct Line {
	points : (Rc<Xy>, Rc<Xy>)
}

struct State {
    // Here we will find the map, player, etc.
    points : Vec<Rc<Xy>>,
    lines : Vec<Line>,
	coordinates_mouse : Xy,
	object : Movable,
}

impl Line {
	fn from_points(points : & Vec<Rc<Xy>>, index1 : usize, index2 : usize) -> Self {
		return Line { points : (Rc::clone(&points[index1]), Rc::clone(&points[index2]))};
	}
	fn draw(& self, ctx: &mut Context) -> GameResult {
		let line = graphics::Mesh::new_line(
			ctx,
			&[*self.points.0, *self.points.1],
			1 as f32,
			[0.0, 0.0, 0.0, 1.0].into()
		)?;
		let draw_params = graphics::DrawParam::new();
			//.dest(Xy(80, 80));
		graphics::draw(ctx, &line, draw_params)?;
		return Ok(());
	}
}




impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ggez::timer::check_update_time(ctx, conf::FPS) {
			//self.object.coordinates.x += 1 as f64;
			self.object.move0();
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

        for line in self.lines.iter() {
    		line.draw(ctx)?;
    	}

		let circle = graphics::Mesh::new_circle(
			ctx,
			graphics::DrawMode::fill(),
			Xy(0, 0),
			10 as f32,
			0.25 as f32,
			[1.0, 1.0, 1.0, 1.0].into()

		)?;
		
		let draw_params_circle = graphics::DrawParam::new()        // This describes the __POSITION__ of created mesh
			.dest(self.object.coordinates);

		graphics::draw(ctx, &circle, draw_params_circle)?;
		graphics::present(ctx)?;                            // Something that helps timer to work
        ggez::timer::yield_now();                           // ...nevermind
        return Ok(());
    }


    fn key_up_event( &mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        // There are also other kyeboard and mouse functions, see:
        // https://docs.rs/ggez/0.5.1/ggez/event/trait.EventHandler.html
        match keycode {
            KeyCode::W => println!("W was pressed..."),
            _ => (),
        }
    }
	fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {

	}
	fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
		self.coordinates_mouse = Xy(x, y);
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
    let points = vec![
    	Rc::new(Xy::new(50, 50)),
    	Rc::new(Xy::new(100, 50)),
    	Rc::new(Xy::new(100, 100)),
    	Rc::new(Xy::new(50, 100))
    ];
    let mut state = State {
    	lines : vec![
    		Line::from_points(&points, 0, 1),
    		Line::from_points(&points, 1, 2),
    		Line::from_points(&points, 2, 3),
    		Line::from_points(&points, 3, 0),
    	],
    	points : points,
		coordinates_mouse: Xy::new(0, 0),
		object: Movable {
			coordinates: Xy(100, 100),
			acceleration_vector: Xy(0, 0),
			movement_vector: Xy(5, 0),
			braking_coefficient : 0.01
		}
	};

    return event::run(ctx, events_loop, &mut state);
}
