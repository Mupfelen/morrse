use yew::prelude::*;
use crate::counter::Counter;

pub struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, _: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id="container">
                <Counter />
                <Counter />
            </div>
        }
    }
}