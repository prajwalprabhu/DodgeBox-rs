use opengl_graphics::{GlGraphics, OpenGL};
use piston::{
    Button, EventLoop, EventSettings, Events, Key, PressEvent, RenderArgs, RenderEvent, UpdateArgs,
    UpdateEvent, WindowSettings,
};
use piston_window::PistonWindow;
use rand::Rng;

struct GameBox {
    coord: [f64; 2],
    size: f64,
}

impl GameBox {
    fn new(size: f64) -> Self {
        let coord = [0.0, 0.0];
        Self { coord, size }
    }
}
struct App {
    gl: GlGraphics,
    window: [f64; 2],
    player: GameBox,
    opponent: GameBox,
}

impl App {
    fn new(gl: GlGraphics, window: [f64; 2], size: f64) -> Self {
        let mut player = GameBox::new(size);
        player.coord = [0.0, window[1] - player.size];
        let opponent = GameBox::new(size);
        Self {
            gl,
            window,
            player,
            opponent,
        }
    }
    fn generate(&self) -> [f64; 2] {
        let mut rng = rand::thread_rng();
        [
            rng.gen_range(0.0..(self.window[1] - self.player.size)),
            // self.window[1] - self.player.size,
            0.0,
        ]
    }
    fn render(&mut self, args: RenderArgs) {
        use graphics::*;
        // self.opponent.coord = self.generate();

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        let player_square =
            rectangle::square(self.player.coord[0], self.player.coord[1], self.player.size);
        let opponent_square = rectangle::square(
            self.opponent.coord[0],
            self.opponent.coord[1],
            self.opponent.size,
        );

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            rectangle(RED, player_square, c.transform, gl);
            rectangle(BLUE, opponent_square, c.transform, gl);
        });
    }
    fn update(&mut self, _args: UpdateArgs) {
        if (self.opponent.coord[1] + self.opponent.size) > self.player.coord[1]
            && (self.opponent.coord[0] > self.player.coord[0]
                && self.opponent.coord[0] < (self.player.size + self.player.coord[0])
                || ((self.opponent.coord[0] + self.opponent.size) > self.player.coord[0])
                    && (self.opponent.coord[0] + self.opponent.size
                        < (self.player.size + self.player.coord[0])))
        {
            println!("Lost ");
        } else {
            // println!("not lost");
        }
        if self.opponent.coord[1] > (self.window[1] - self.opponent.size) {
            self.opponent.coord = self.generate();
            // self.opponent.coord = [55.0, 0.0];
        } else {
            self.opponent.coord[1] += 5.0;
        }
    }
    fn press(&mut self, key: Button) {
        match key {
            Button::Keyboard(Key::Left) => {
                if self.player.coord[0] > 0.0 {
                    self.player.coord[0] -= 5.0;
                }
            }
            Button::Keyboard(Key::Right) => {
                if self.player.coord[0] < self.window[1] {
                    self.player.coord[0] += 5.0;
                }
            }
            _ => {}
        }
    }
}

fn main() {
    const WINDOW: [f64; 2] = [200.0, 200.0];
    const FPS: u64 = 05;
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let mut window: PistonWindow = WindowSettings::new("Dodge Box", WINDOW)
        .exit_on_esc(true)
        .build()
        .expect("Failed load window");
    let mut glyphs = window
        .load_font(assets.join("FiraSans-Regular.ttf"))
        .unwrap();
    let mut app = App::new(GlGraphics::new(OpenGL::V3_2), WINDOW, 50.0);
    let mut events = Events::new(EventSettings::new());
    events.max_fps(FPS);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(args);
        }

        if let Some(args) = e.update_args() {
            app.update(args);
        }
        if let Some(key) = e.press_args() {
            app.press(key);
        }
    }
    println!("Hello world");
}
