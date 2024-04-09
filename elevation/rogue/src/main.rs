use tcod::{
    colors::*,
    console::*,
    input::{ Key, KeyCode::* },
};

const SCREEN_WIDTH: f64 = 80.;
const SCREEN_HEIGHT: f64 = 50.;
const LIMIT_FPS: i32 = 60;

struct Player {
    x: f64,
    y: f64,
}

impl Player {
    pub fn new(_x: f64, _y: f64) -> Player {
        Player {
            x: _x,
            y: _y,
        }
    }
}

struct Tcod {
    root: Root,
}

impl Tcod {
    pub fn new() -> Tcod {
        let _root = Root::initializer()
            .font("./assets/arial10x10.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
            .title("ROGUE")
            .init();
        Tcod {
            root: _root,
        }
    }
}

fn main() {
    let mut player = Player::new(SCREEN_WIDTH/2., SCREEN_HEIGHT/2.);
    let mut tcod = Tcod::new();
    tcod::system::set_fps(LIMIT_FPS);
    while !tcod.root.window_closed() {
        tcod.root.set_default_foreground(WHITE);
        tcod.root.clear();
        tcod.root.put_char(1, 1, '@', BackgroundFlag::None);
        tcod.root.flush();
        tcod.root.wait_for_keypress(true);
    }
}

fn handle_keys(tcod: &mut Tcod, player: &mut Player) -> bool {
    let key = tcod.root.wait_for_keypress(true);
    match key {
        Key { code: Up, .. }   => player.y -= 1.,
        Key { code: Down, .. } => player.y += 1.,
        Key { code: Left, .. } => player.x -= 1.,
        Key { code: Right, .. }=> player.x += 1.,

        _ => {},
    }

    false
}
