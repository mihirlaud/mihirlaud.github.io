#![recursion_limit = "2048"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod footer;
mod header;
mod resume;

use crate::footer::Footer;
use crate::header::Header;
use crate::resume::Resume;

struct Main {
    link: ComponentLink<Self>,
}

enum Msg {}

impl Component for Main {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Header />
                <Resume />
                //<Carousel />
                <Footer />
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Main>::new().mount_to_body();
}
