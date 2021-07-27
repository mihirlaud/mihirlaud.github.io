use yew::prelude::*;

pub struct Footer {
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct FooterProps {}

pub enum Msg {}

impl Component for Footer {
    type Message = Msg;
    type Properties = FooterProps;

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
