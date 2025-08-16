use leptos::prelude::*;
use reactive_stores::Store;

#[derive(Clone, Debug, Default)]
enum BlockType {
    #[default]
    OrangeRicky,
    BlueRicky,
    Hero,
    Teewee,
    ClevelandZ,
    RhodeIslandZ,
    Smashboy,
}

#[derive(Clone, Debug, Default, Store)]
struct GlobalState {
    playfield: [[u32; 10]; 20],
    current_block: BlockType,
}

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div>
            "Hello World"
        </div>
    }
}
