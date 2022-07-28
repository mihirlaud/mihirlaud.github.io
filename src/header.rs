use yew::prelude::*;

pub struct Header {
}

#[derive(Properties, Clone, PartialEq)]
pub struct HeaderProps {}

pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = HeaderProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {  }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="header">
                <div class="header-left">
                    <h1>{"Mihir Laud"}</h1>
                    <h2>{"Aeronautical & Astronautical Engineering Student"}</h2>
                    <h2>{"Purdue University Class of 2024"}</h2>
                </div>

                <div class="header-right">
                    <h4>{"mihirlaud@gmail.com"}</h4>
                    <h4>{"732.742.4653"}</h4>
                    <h4>{"GitHub: mihirlaud"}</h4>
                    <h4>{"Work Status: U.S. Citizen"}</h4>
                </div>
            </div>
        }
    }
}
