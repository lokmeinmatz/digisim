#![feature(drain_filter)]

use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::conf::{WindowMode, WindowSetup, NumSamples};
use cgmath::{Vector2, Point2};

use ggez::input::mouse::MouseButton;


mod interval;
mod ecs;
mod entities;
use interval::Timer;
use ggez::graphics::{DrawParam, DrawMode, Rect, Color};
use crate::entities::World;
use std::path;

const SECOND_UPDATE: usize = 123;
pub type Vec2 = Vector2<f32>;

fn main() -> GameResult<()> {
    // Make a Context.
    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (mut ctx, mut event_loop) = ContextBuilder::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_AUTHORS"))
        .window_mode(WindowMode {
            width: 1920.0,
            height: 1080.0,
            resizable: true,
            .. WindowMode::default()
        })
        .window_setup(WindowSetup {
            title: "ChessRS".to_owned(),
            samples: NumSamples::Four,
            .. WindowSetup::default()
        })
        .add_resource_path(resource_dir)
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = Sim::new(&mut ctx)?;

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    };

    Ok(())
}

struct Sim {
    last_mouse_pos: Vec2,
    timer: Timer,
    world: World
}

impl Sim {
    pub fn new(ctx: &mut Context) -> GameResult<Sim> {
        // Load/create resources such as images here.


        let mut timer = Timer::new();

        timer.add(SECOND_UPDATE, 1.0, true);

        Ok(Sim {
            last_mouse_pos: Vector2::new(0.0, 0.0),
            timer,
            world: World::new(ctx)
        })
    }
}

impl EventHandler for Sim {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Update code here...

        for timer_id in self.timer.update(ggez::timer::delta(ctx).as_secs_f32()) {
            match timer_id {
                SECOND_UPDATE => {
                    println!("FPS: {}", ggez::timer::fps(ctx));
                },
                _ => {}
            }
        }



        std::thread::yield_now();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let (w, h) = graphics::drawable_size(ctx);

        let scale = w.min(h) / 8.0;

        let offset = ((w / 8.0 - scale) * 4.0, (h / 8.0 - scale) * 4.0);

        let param = DrawParam::default().scale(Vec2::new(scale, scale)).dest(cgmath::Point2::from(offset));
        //let param = DrawParam::default();
        graphics::push_transform(ctx, Some(param.to_matrix()));
        graphics::set_transform(ctx, param.to_matrix());
        graphics::apply_transformations(ctx);

        if ggez::timer::ticks(ctx) % 100 == 0 {
            println!("w {} h {}", w, h);
            println!("scale {} offset {:?}", scale, offset);
        }

        self.world.render(ctx)?;
        graphics::pop_transform(ctx);
        graphics::present(ctx)
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let new_rect = graphics::Rect::new(
            0.0,
            0.0,
            width,
            height,
        );
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        let m_pos = Vec2::new(x, y);
        self.last_mouse_pos = m_pos;
    }

}