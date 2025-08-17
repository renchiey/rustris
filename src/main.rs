use leptos::{ev::keydown, logging, prelude::*};
use leptos_use::use_event_listener;
use rand::Rng;
use reactive_stores::Store;

use crate::blocks::{
    get_blue_start, get_cleveland_start, get_color, get_hero_start, get_orange_start,
    get_rhode_start, get_smash_start, get_teewee_start,
};
mod blocks;

const PLAYFIELD_HEIGHT: usize = 24;
const PLAYFIELD_WIDTH: usize = 10;

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

#[derive(Clone, Debug, Default, PartialEq)]
#[allow(dead_code)]
struct Position(i8, i8);

#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
struct Block {
    pub block_type: BlockType,
    pub orientation: Orientation,
    pub blocks: [Position; 4],
}

impl Block {
    fn get_random() -> Self {
        let mut rng = rand::rng();

        let random_in_range = rng.random_range(1..=7);

        let (block_type, blocks) = match random_in_range {
            1 => (BlockType::OrangeRicky, get_orange_start()),
            2 => (BlockType::BlueRicky, get_blue_start()),
            3 => (BlockType::Hero, get_hero_start()),
            4 => (BlockType::Teewee, get_teewee_start()),
            5 => (BlockType::ClevelandZ, get_cleveland_start()),
            6 => (BlockType::RhodeIslandZ, get_rhode_start()),
            7 => (BlockType::Smashboy, get_smash_start()),
            _ => panic!("Not supposed to be here"),
        };

        Block {
            block_type,
            orientation: Orientation::Up,
            blocks,
        }
    }

    // rotation is performed by using the first block in the blocks array as the pivot
    fn rotate(&mut self, playfield: &[[BlockType; PLAYFIELD_WIDTH]; PLAYFIELD_HEIGHT]) -> Self {
        let pivot_point = &self.blocks[0];

        let transformation = self.blocks.clone().map(|point| {
            // translate to origin
            let translated = Position(point.0 - pivot_point.0, point.1 - pivot_point.1);

            // rotate 90° clockwise: (x, y) -> (y, -x)
            let rotated = Position(translated.1, -translated.0);

            // translate back
            Position(rotated.0 + pivot_point.0, rotated.1 + pivot_point.1)
        });

        // check for collisions
        for point in transformation.iter() {
            let row: usize = match point.0.try_into() {
                Ok(r) => r,
                Err(_) => return self.to_owned(),
            };

            let col: usize = match point.1.try_into() {
                Ok(c) => c,
                Err(_) => return self.to_owned(),
            };

            if row >= PLAYFIELD_HEIGHT || col >= PLAYFIELD_WIDTH {
                return self.to_owned();
            }

            if !matches!(playfield[row][col], BlockType::None) {
                return self.to_owned();
            }
        }

        self.blocks = transformation;
        self.to_owned()
    }

    fn descend(&mut self, playfield: &[[BlockType; PLAYFIELD_WIDTH]; PLAYFIELD_HEIGHT]) -> Self {
        let transformation = self
            .blocks
            .clone()
            .map(|point| Position(point.0 + 1, point.1));

        for point in transformation.iter() {
            if point.0 >= playfield.len() as i8 {
                return self.to_owned();
            }
        }

        self.blocks = transformation;
        self.to_owned()
    }

    fn move_left(&mut self, playfield: &[[BlockType; PLAYFIELD_WIDTH]; PLAYFIELD_HEIGHT]) -> Self {
        let transformation = self
            .blocks
            .clone()
            .map(|point| Position(point.0, point.1 - 1));

        for point in transformation.iter() {
            if point.1 < 0 {
                return self.to_owned();
            }
        }

        self.blocks = transformation;
        self.to_owned()
    }

    fn move_right(&mut self, playfield: &[[BlockType; 10]; 24]) -> Self {
        let transformation = self
            .blocks
            .clone()
            .map(|point| Position(point.0, point.1 + 1));

        for point in transformation.iter() {
            if point.1 >= PLAYFIELD_WIDTH as i8 {
                return self.to_owned();
            }
        }

        self.blocks = transformation;
        self.to_owned()
    }
}

#[derive(Clone, Debug, Default, Store)]
#[allow(dead_code)]
struct GlobalState {
    pub playfield: [[BlockType; 10]; 24],
    pub active_block: Block,
}

impl GlobalState {
    fn new() -> Self {
        GlobalState {
            playfield: [[BlockType::None; PLAYFIELD_WIDTH]; PLAYFIELD_HEIGHT],
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
    let (active_block, set_active_block) = signal(Block::get_random());

    use_event_listener(
        document().body(),
        keydown,
        move |evt: leptos::ev::KeyboardEvent| {
            logging::log!("key press: {}", &evt.key());

            let playfield = &state.get().playfield;
            let mut block = active_block.get();

            if &evt.key() == "ArrowUp" {
                set_active_block.set(block.rotate(playfield));
            }

            if &evt.key() == "ArrowDown" {
                set_active_block.set(block.descend(playfield));
            }

            if &evt.key() == "ArrowRight" {
                set_active_block.set(block.move_right(playfield));
            }

            if &evt.key() == "ArrowLeft" {
                set_active_block.set(block.move_left(playfield));
            }
        },
    );

    view! {
      <div class:playfield>
        {move || {
          state
            .get()
            .playfield
            .into_iter()
            .enumerate()
            .map(|(r_i, r)| {
              view! {
                <div class:row>
                  {r
                    .into_iter()
                    .enumerate()
                    .map(|(c_i, _)| {
                      view! {
                        <div class="cell">
                          {move || {
                            if active_block.get().blocks.contains(&Position(r_i as i8, c_i as i8)) {
                              Some(

                                view! {
                                  <div
                                    class="block"
                                    style=(
                                      "background-color",
                                      move || get_color(active_block.get().block_type),
                                    )
                                  ></div>
                                },
                              )
                            } else {
                              None
                            }
                          }}
                        </div>
                      }
                    })
                    .collect_view()}
                </div>
              }
            })
            .collect_view()
        }}
      </div>
    }
}
