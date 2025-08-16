use leptos::{ev::keydown, logging, prelude::*};
use leptos_use::use_event_listener;
use rand::Rng;
use reactive_stores::Store;

#[derive(Clone, Copy, Debug, Default)]
enum BlockType {
    #[default]
    None,
    OrangeRicky,
    BlueRicky,
    Hero,
    Teewee,
    ClevelandZ,
    RhodeIslandZ,
    Smashboy,
}

#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
enum Orientation {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
struct Position(u8, u8);

#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
struct Block {
    pub block_type: BlockType,
    pub centre_pos: Position,
    pub orientation: Orientation,
    pub blocks: [Position; 4],
}

impl Block {
    fn get_random() -> Self {
        let mut rng = rand::rng();

        let random_in_range = rng.random_range(1..=7);

        let block_type = match random_in_range {
            1 => BlockType::OrangeRicky,
            2 => BlockType::BlueRicky,
            3 => BlockType::Hero,
            4 => BlockType::Teewee,
            5 => BlockType::ClevelandZ,
            6 => BlockType::RhodeIslandZ,
            7 => BlockType::Smashboy,
            _ => panic!("Not supposed to be here"),
        };

        Block {
            block_type,
            centre_pos: Position(0, 4),
            orientation: Orientation::Up,
        }
    }

    fn rotate_right(&self, playfield: &[[BlockType; 10]; 20]) {}

    fn default() -> Self {
        Block {
            block_type: BlockType::None,
            centre_pos: Position(0, 0),
            orientation: Orientation::Up,
        }
    }
}

#[derive(Clone, Debug, Default, Store)]
#[allow(dead_code)]
struct GlobalState {
    pub playfield: [[BlockType; 10]; 20],
    pub active_block: Block,
}

impl GlobalState {
    fn new() -> Self {
        GlobalState {
            playfield: [[BlockType::None; 10]; 20],
            active_block: Block::get_random(),
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    console_error_panic_hook::set_once();
    logging::log!("csr mode - mounting to body");

    view! {
        <div class:container>
            <h1>"Tetris"</h1>
            <Game />
        </div>
    }
}

#[component]
fn Game() -> impl IntoView {
    let (state, set_state) = signal(GlobalState::new());

    use_event_listener(
        document().body(),
        keydown,
        |evt: leptos::ev::KeyboardEvent| {
            logging::log!("key press: {}", &evt.key());
        },
    );

    view! {
        <div class:playfield>
            {move || {
                state
                    .get()
                    .playfield
                    .into_iter()
                    .map(|r| {
                        view! {
                            <div class:row>
                                {r
                                    .into_iter()
                                    .map(|_| view! { <div class:cell></div> })
                                    .collect_view()}
                            </div>
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}
