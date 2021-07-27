use yew::prelude::*;

pub struct Carousel {
    link: ComponentLink<Self>,
    slides: Vec<Slide>,
    selected: usize,
}

#[derive(Properties, Clone, PartialEq)]
pub struct CarouselProps {}

pub enum CarouselMsg {
    MoveLeft,
    MoveRight,
}

impl Component for Carousel {
    type Properties = CarouselProps;
    type Message = CarouselMsg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let arms = Slide {
            src: "images/arms.jpg".to_string(),
            title: "ARMS".to_string(),
            description: "A library for programming VEX robot chassis".to_string(),
            link: Some("https://github.com/purduesigbots/arms".to_string()),
        };

        let lingua = Slide {
            src: "images/lingua.jpg".to_string(),
            title: "Lingua".to_string(),
            description: "A programming language made with beginners in mind".to_string(),
            link: Some("https://github.com/mihirlaud/lingua".to_string()),
        };

        let auto_rocket = Slide {
            src: "images/auto_rocket.jpg".to_string(),
            title: "auto-rocket".to_string(),
            description: "A MATLAB control simulation for a self-landing rocket".to_string(),
            link: Some("https://github.com/mihirlaud/auto-rocket".to_string()),
        };

        let spacex_controls = Slide {
            src: "images/spacex.jpg".to_string(),
            title: "SpaceX ISS Docking Controls".to_string(),
            description: "Firefox extension for autonomously docking with SpaceX ISS Simulator"
                .to_string(),
            link: Some("https://github.com/mihirlaud/spacex-iss-controls".to_string()),
        };

        let slides = vec![arms, lingua, auto_rocket, spacex_controls];

        Self {
            link,
            slides,
            selected: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            CarouselMsg::MoveLeft => {
                if self.selected == 0 {
                    self.selected = self.slides.len() - 1;
                } else {
                    self.selected -= 1;
                }
            }
            CarouselMsg::MoveRight => {
                if self.selected == self.slides.len() - 1 {
                    self.selected = 0;
                } else {
                    self.selected += 1;
                }
            }
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html!(
            <div class="carousel-wrapping" >
                <h2 class="section-name">{"Projects"}</h2>
                <p class="section-description">{"These are some of the personal projects I have worked on throughout the years. Feel free to click on the image to explore the source code directly on GitHub."}</p>

                <div class="carousel">
                    {for (0 .. self.slides.len()).map(|index| {
                        let slide = self.slides.get(index).unwrap();
                        let hidden = if index == self.selected { "selected" } else { "hidden" };
                        html!(
                            <div class={format!("carousel-slide {}", hidden)}>
                                <h2 class="carousel-title" >{slide.title.as_str()}</h2>
                                <a target="_blank" href={if slide.link.is_some() { String::from(slide.link.as_ref().unwrap().as_str()) } else {String::from("")}}>
                                    <img class="carousel-image" src={String::from(slide.src.as_str())} />
                                </a>
                                <h3 class="carousel-desc" >{slide.description.as_str()}</h3>
                            </div>
                        )
                    })}
                    <button class="carousel-button" id="left-button" onclick={self.link.callback(|_| CarouselMsg::MoveLeft)}>{"❮"}</button>
                    <button class="carousel-button" id="right-button" onclick={self.link.callback(|_| CarouselMsg::MoveRight)}>{"❯"}</button>
                </div>
            </div>
        )
    }
}

#[derive(Clone)]
struct Slide {
    src: String,
    title: String,
    description: String,
    link: Option<String>,
}
