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

                <div id="header-left">
                    <img id="headshot" src="images/headshot.jpeg"/>
                </div>

                <div id="header-right">
                    <h1>{"Mihir Laud"}</h1>
                    <h2>{"Aeronautical & Astronautical Engineering Student"}</h2>
                    <h3>{"Purdue University, Class of 2024"}</h3>
                    <h4>{"Work Status: U.S. Citizen"}</h4>

                    <button class="header-button">
                        <a href="https://github.com/mihirlaud" target="_blank">
                            <img class="footer-img" src="images/github.png"/>
                        </a>
                    </button>

                    <button class="header-button">
                        <a href="https://www.linkedin.com/in/mihirlaud" target="_blank">
                            <img class="footer-img" src="images/linkedin.png"/>
                        </a>
                    </button>

                    <button class="header-button">
                        <a href="mailto:mihirlaud@gmail.com" target="_blank">
                            <img class="footer-img" src="images/email.png"/>
                        </a>
                    </button>
                </div>
            </div>
        }
    }
}
