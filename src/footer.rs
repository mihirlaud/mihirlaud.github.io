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
                <button class="footer-button">{"GitHub"}</button>
                <button class="footer-button">{"LinkedIn"}</button>
                <button class="footer-button">{"Email"}</button>
            </div>
        }
    }
}
