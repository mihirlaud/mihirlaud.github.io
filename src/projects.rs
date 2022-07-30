use yew::prelude::*;

pub struct Projects {
}

#[derive(Properties, PartialEq, Clone)]
pub struct ProjectsProps {}

pub enum ProjectsMsg {}

impl Component for Projects {
    type Properties = ProjectsProps;
    type Message = ProjectsMsg;

    fn create(_ctx: &Context<Self>) -> Self {
        Projects { }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {

        let ros_virtual_desc = "As a research assistant in the AIMS Lab, I designed and tested a flight controller for a quadrotor using ROS. This is a video demonstration of the controller moving to multiple specified points in the Gazebo virtual testing environment. ".to_string();
        let ros_virtual_url = "https://www.youtube.com/embed/-N6l_FVbApk".to_string();

        let ros_mocap_desc = "The following semester, I was tasked with integrating the lab's motion capture setup with the node so that the flight controller could be tested in a physical space. The video shows how the two nodes interact over Ethernet to provide the controller with real-time positional data of the quadrotor, which it then uses to issue motor commands to fly to the desired place.";
        let ros_mocap_url = "https://www.youtube.com/embed/noiL649VDvA".to_string();

        let prog_skills_desc = "In May 2022, Purdue SIGBots won World Skills Champion in the VEX Robotics Competition for the Tipping Point Season. This is a recording of our autonomous skills run, painstakingly crafted over 10 months by our small team of dedicated programmers. I am proud to have been Software Lead for SIGBots during this season, and I very much look forward to competing on the team in future years.".to_string();
        let prog_skills_url = "https://www.youtube.com/embed/NSte89GYpzA".to_string();

        html! {
            <div class="projects">
                <div id="project-header">
                    <h2 class="section-name">{"Projects"}</h2>
                    <p class="section-description">{"These are some of the projects I have worked on throughout the years. Feel free to click on the image to explore the source code directly on GitHub."}</p>
                </div>

                <Project name="ROS Pixhawk Controller" desc={ros_virtual_desc} url={ros_virtual_url} left={true} />
                <Project name="ROS Mo-Cap Integration" desc={ros_mocap_desc} url={ros_mocap_url} left={false} />
                <Project name="SIGBots Programming Skills" desc={prog_skills_desc} url={prog_skills_url} left={true} />
            </div>
        }
    }
}

struct Project {
}

#[derive(Properties, Clone, PartialEq)]
struct ProjectProps {
    name: String,
    desc: String,
    url: String,
    left: bool,
}

enum ProjectMsg {}

impl Component for Project {
    type Properties = ProjectProps;
    type Message = ProjectMsg;

    fn create(_ctx: &Context<Self>) -> Self {
        Project {
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let name = &ctx.props().name;
        let desc = &ctx.props().desc;
        let url = &ctx.props().url;
        let left = &ctx.props().left;

        html! {
            <div class="project">
                <div class="project-content">
                    <div class="project-text" style={format!("float: {}", if *left { "left" } else { "right" })}>
                        <h2 class="project-name">{name.as_str()}</h2>
                        <p class="section-description">{desc.as_str()}</p>
                    </div>
                    <div class="project-video">
                        <iframe width="100%" height="315" src={format!("{}", url)} title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen={true}></iframe>
                    </div>
                </div>
            </div>
        }
    }
}
