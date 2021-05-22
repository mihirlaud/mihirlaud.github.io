use yew::prelude::*;

pub struct Header {
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct HeaderProps {}

pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = HeaderProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
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
