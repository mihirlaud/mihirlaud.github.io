#![recursion_limit = "2048"]
use yew::prelude::*;

mod carousel;
mod footer;
mod header;
mod resume;

use carousel::Carousel;
use footer::Footer;
use header::Header;
use resume::Resume;

enum Msg {
}

struct Model {
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <Header />
                <Resume />
                <Carousel />
                <Footer />
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}