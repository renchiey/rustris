use std::{cell::Cell, rc::Rc};

use leptos::{
    ev::{keydown, keyup},
    logging::{self},
    prelude::*,
};
use leptos_use::{use_event_listener, use_interval_fn};
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
    pub projection: Option<[Position; 4]>,
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
            projection: None,
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
            if point.1 < 0
                || (point.1 > 0
                    && !matches!(
                        playfield[point.0 as usize][point.1 as usize],
                        BlockType::None,
                    ))
            {
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
            if point.1 >= PLAYFIELD_WIDTH as i8
                || (point.1 < (PLAYFIELD_WIDTH - 1) as i8
                    && !matches!(
                        playfield[point.0 as usize][point.1 as usize],
                        BlockType::None,
                    ))
            {
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
    pub projection_blocks: Option<[Position; 4]>,
    pub game_over: bool,
    pub game_paused: bool,
}

impl GlobalState {
    fn new() -> Self {
        GlobalState {
            playfield: [[BlockType::None; PLAYFIELD_WIDTH]; PLAYFIELD_HEIGHT],
            active_block: Block::get_random(),
            game_over: false,
            projection_blocks: None,
            game_paused: false,
        }
    }

    fn rotate_block(&mut self) -> Self {
        self.active_block = self.active_block.rotate(&self.playfield);

        self.get_projection();

        self.to_owned()
    }

    fn descend_block(&mut self) -> Self {
        if self.check_collision() {
            return self.to_owned();
        };

        self.active_block = self.active_block.descend(&self.playfield);
        self.get_projection();
        self.to_owned()
    }

    fn get_projection(&mut self) {
        let mut transformations = self.active_block.blocks.clone();

        for point in transformations.iter() {
            if point.0 == (PLAYFIELD_HEIGHT - 1) as i8 {
                return;
            }
        }

        loop {
            for p in transformations.iter_mut() {
                p.0 += 1;
            }
            let mut complete = false;

            for point in transformations.iter() {
                if point.0 == (PLAYFIELD_HEIGHT - 1) as i8
                    || !matches!(
                        self.playfield[(point.0 + 1) as usize][point.1 as usize],
                        BlockType::None
                    )
                {
                    complete = true;
                    break;
                }
            }
            if complete {
                self.projection_blocks = Some(transformations);
                break;
            }
        }
    }

    fn full_descend_block(&mut self) -> Self {
        if !self.check_collision() {
            self.active_block.descend(&self.playfield);

            return self.full_descend_block();
        }

        self.place_block()
    }

    fn move_block_left(&mut self) -> Self {
        self.active_block = self.active_block.move_left(&self.playfield);
        self.get_projection();
        self.to_owned()
    }

    fn move_block_right(&mut self) -> Self {
        self.active_block = self.active_block.move_right(&self.playfield);
        self.get_projection();

        self.to_owned()
    }

    fn place_block(&mut self) -> Self {
        let block_type = self.active_block.block_type;
        self.active_block
            .blocks
            .iter()
            .for_each(|point| self.playfield[point.0 as usize][point.1 as usize] = block_type);

        self.update_rows();

        if self.check_game_over() {
            self.game_over = true;
        } else {
            self.active_block = Block::get_random();
        }

        self.to_owned()
    }

    fn check_collision(&self) -> bool {
        let playfield = self.playfield;
        for point in self.active_block.blocks.iter() {
            if point.0 == (PLAYFIELD_HEIGHT - 1) as i8
                || !matches!(
                    playfield[(point.0 + 1) as usize][point.1 as usize],
                    BlockType::None
                )
            {
                return true;
            }
        }

        false
    }

    fn update_rows(&mut self) -> Self {
        let mut r = PLAYFIELD_HEIGHT - 1;

        while r >= 4 {
            let row_incomplete = self.playfield[r]
                .iter()
                .any(|cell| matches!(cell, BlockType::None));

            if !row_incomplete {
                self.shift_down_from_row(r);
            } else {
                r -= 1;
            }
        }

        self.to_owned()
    }

    fn shift_down_from_row(&mut self, row: usize) {
        let mut r = row;
        while r >= 4 {
            self.playfield[r] = self.playfield[r - 1];
            r -= 1;
        }
        self.playfield[r] = [BlockType::None; PLAYFIELD_WIDTH];
    }

    fn check_game_over(&self) -> bool {
        for point in self.active_block.blocks.iter() {
            if point.0 < 4 {
                return true;
            }
        }

        false
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
        <h1 class="title">"RUSTRIS"</h1>
        <p>"Tetris built in Rust!"</p>
        <Game />
      </div>
    }
}

#[component]
fn Game() -> impl IntoView {
    let (state, set_state) = signal(GlobalState::new());

    let block_place_timer = Rc::new(Cell::new(0));

    // interval for block descension
    // wrap it in a reference counter so that we can move a copy into each of the event listener
    // closures
    let block_descend_interval = Rc::new(use_interval_fn(
        move || {
            set_state.set(state.get().descend_block());
        },
        500,
    ));

    let block_descend_clone = Rc::clone(&block_descend_interval);
    let _ = use_event_listener(
        document().body(),
        keydown,
        move |evt: leptos::ev::KeyboardEvent| {
            logging::log!("key press: {}", &evt.key());

            let mut s = state.get();

            if s.game_over {
                return;
            }

            if !s.game_paused {
                if &evt.key() == "ArrowUp" {
                    set_state.set(s.rotate_block());
                }

                if &evt.key() == "ArrowDown" {
                    (block_descend_clone.pause)();
                    set_state.set(s.descend_block());
                }

                if &evt.key() == "ArrowRight" {
                    set_state.set(s.move_block_right());
                }

                if &evt.key() == "ArrowLeft" {
                    set_state.set(s.move_block_left());
                }

                if &evt.key() == " " {
                    set_state.set(s.full_descend_block());
                }
            }

            if &evt.key() == "Escape" {
                if state.get().game_paused {
                    logging::log!("Game unpaused.");
                    (block_descend_clone.resume)();
                } else {
                    logging::log!("Game paused.");
                    (block_descend_clone.pause)();
                }
                set_state.write().game_paused = !state.get().game_paused;
            }
        },
    );

    let _ = use_event_listener(
        document().body(),
        keyup,
        move |evt: leptos::ev::KeyboardEvent| {
            if &evt.key() == "ArrowDown" && !state.get().game_paused {
                (block_descend_interval.resume)();
            }
        },
    );

    // interval to check for collisions
    let _ = use_interval_fn(
        move || {
            if state.get().check_collision() {
                block_place_timer.set(block_place_timer.get() + 1);

                if block_place_timer.get() == 5 {
                    set_state.set(state.get().place_block());
                    block_place_timer.set(0);
                }
            } else {
                block_place_timer.set(0);
            }
        },
        100,
    );

    view! {
      <div class="playfield">
        {move || if state.get().game_paused { Some(view! { <PauseScreen /> }) } else { None }}
        {move || {
          state
            .get()
            .playfield
            .iter()
            .enumerate()
            .map(|(r_i, r)| {
              if r_i >= 4 {
                Some(
                  view! {
                    <div class:row>
                      {r
                        .iter()
                        .enumerate()
                        .map(|(c_i, _)| {
                          view! {
                            <div class="cell">
                              {move || {
                                let active_block = state.get().active_block;
                                if active_block.blocks.contains(&Position(r_i as i8, c_i as i8)) {
                                  Some(

                                    view! {
                                      <div
                                        class="block"
                                        style=(
                                          "background-color",
                                          move || get_color(active_block.block_type),
                                        )
                                      ></div>
                                    }
                                      .into_any(),
                                  )
                                } else if !matches!(
                                  state.get().playfield[r_i][c_i],
                                  BlockType::None
                                ) {
                                  Some(
                                    view! {
                                      <div
                                        class="block"
                                        style=(
                                          "background-color",
                                          move || get_color(state.get().playfield[r_i][c_i]),
                                        )
                                      ></div>
                                    }
                                      .into_any(),
                                  )
                                } else if state.get().projection_blocks.is_some()
                                  && state
                                    .get()
                                    .projection_blocks
                                    .unwrap()
                                    .contains(&Position(r_i as i8, c_i as i8))
                                {
                                  Some(
                                    view! {
                                      <div
                                        class="projection"
                                        style=(
                                          "background-color",
                                          move || get_color(active_block.block_type),
                                        )
                                      ></div>
                                    }
                                      .into_any(),
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
                  },
                )
              } else {
                None
              }
            })
            .collect_view()
        }}
      </div>
    }
}

#[component]
fn PauseScreen() -> impl IntoView {
    view! {
      <div class="pause-container">
        <p class="pause-text">Game Paused</p>
      </div>
    }
}
