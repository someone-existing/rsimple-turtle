use ggez::event::{self, EventHandler};
use ggez::{graphics, Context, ContextBuilder, GameResult};

mod utils;

const FILE_NAME: &str = "instructions.trtl"; // Instructions file, change if renamed

// Window data
const WIN_WIDTH: f32 = 800.0;
const WIN_HEIGHT: f32 = 600.0;
const WIN_TITLE: &str = "Turtle";

struct XY {
    x: f32,
    y: f32,
}

struct MainState {
    running: bool, // Used to stop the turtle from moving when all instructions have been completed.
    pos: XY,       // Turtle's current x and y position
    size: XY,      // Turtle's width and height
    direction: XY, // Direction of movement
    speed: f32,
}

impl MainState {
    pub fn new() -> MainState {
        MainState {
            running: true,
            pos: XY {
                x: WIN_WIDTH / 2.0,
                y: WIN_HEIGHT / 2.0,
            },
            size: XY { x: 20.0, y: 20.0 },
            direction: XY { x: 0.0, y: 0.0 },
            speed: 1.0,
        }
    }

    fn walk(&mut self, distance: String, ctx: &mut Context) {
        // The distance is given to the function from the file as a string, so it needs to be converted into a number.
        let step: f32 = distance.parse().expect("Not a number.");

        for _ in 1..step as i32 {
            self.pos.x += self.direction.x * self.speed;
            self.pos.y += self.direction.y * self.speed;
            self.draw(ctx).unwrap();
        }

        // Reset direction after walking
        self.direction.x = 0.0;
        self.direction.y = 0.0;
    }

    fn rotate(&mut self, rotation: String) {
        // Direction x and y represent multipliers for the speed
        match rotation.as_str() {
            "north" => self.direction.y = -1.0,

            "west" => self.direction.x = -1.0,

            "south" => self.direction.y = 1.0,

            "east" => self.direction.x = 1.0,

            _ => panic!("Unknown rotation."),
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        use utils::*;

        let file = read_file(FILE_NAME); // Contents of the instructions file

        if self.running {
            for i in 1..num_of_lines(&file) + 1 {
                // Match the first word of each line.
                match get_nth_word(&get_nth_line(&file, i), 1).as_str() {
                    "rotation" => self.rotate(get_nth_word(&get_nth_line(&file, i), 2)),
                    "walk" => self.walk(get_nth_word(&get_nth_line(&file, i), 2), ctx),
                    "" => (), // Skip empty lines
                    _ => panic!("Unknown command."),
                }
            }
            self.running = false;
            println!("Instructions completed.");
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        // Create the turtle mesh.
        let turtle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect {
                x: self.pos.x,
                y: self.pos.y,
                w: self.size.x,
                h: self.size.y,
            },
            graphics::BLACK,
        )?;

        // Draw turtle to context.
        graphics::draw(ctx, &turtle, graphics::DrawParam::default())?;

        // Present context.
        graphics::present(ctx)
    }
}

fn main() {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("rsimple_turtle", "someone_existing")
        .window_setup(ggez::conf::WindowSetup::default().title(WIN_TITLE))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WIN_WIDTH, WIN_HEIGHT))
        .build()
        .expect("Could not create ggez context.");

    // Create an instance of the event handler.
    let mut main_state = MainState::new();
    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut main_state) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}
