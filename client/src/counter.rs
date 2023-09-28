use yew::{Component, Context, Html};
use yew::prelude::*;

pub struct Counter {
    counter: i32
}

pub enum CounterMsg {
    Increment,
    Decrement,
    Reset
}

impl Component for Counter {
    type Message = CounterMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { counter: 0}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CounterMsg::Increment => {
                self.counter += 1;
            }
            CounterMsg::Decrement => {
                self.counter -= 1;
            }
            CounterMsg::Reset => {
                self.counter = 0;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="cont">
                <button class="counterBtn" onclick={ctx.link().callback(|_| CounterMsg::Increment)}>{ "+1" }</button>
                <p>{ self.counter }</p>
                <button class="counterBtn" onclick={ctx.link().callback(|_| CounterMsg::Decrement)}>{ "-1" }</button>
                <button class="counterBtn" onclick={ctx.link().callback(|_| CounterMsg::Reset)}>{ "Reset" }</button>
            </div>
        }
    }
}