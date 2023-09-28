mod app;
mod counter;

use app::Model;

fn main() {
    yew::Renderer::<Model>::new().render();
}
