use yew::prelude::*;

pub struct Footer {
}

#[derive(Properties, Clone, PartialEq)]
pub struct FooterProps {}

pub enum Msg {}

impl Component for Footer {
    type Message = Msg;
    type Properties = FooterProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {  }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="footer">
                <button class="footer-button">
                    <a href="https://github.com/mihirlaud" target="_blank">
                        <img class="footer-img" src="images/github.png"/>
                    </a>
                </button>

                <button class="footer-button">
                    <a href="https://www.linkedin.com/in/mihirlaud" target="_blank">
                        <img class="footer-img" src="images/linkedin.png"/>
                    </a>
                </button>

                <button class="footer-button">
                    <a href="mailto:mihirlaud@gmail.com" target="_blank">
                        <img class="footer-img" src="images/email.png"/>
                    </a>
                </button>
            </div>
        }
    }
}
