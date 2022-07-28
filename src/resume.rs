use yew::prelude::*;

pub struct Resume {
}

#[derive(Properties, PartialEq, Clone)]
pub struct ResumeProps {}

pub enum ResumeMsg {}

impl Component for Resume {
    type Properties = ResumeProps;
    type Message = ResumeMsg;

    fn create(_ctx: &Context<Self>) -> Self {
        Resume { }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let objective = SectionContent {
            title: "".to_string(),
            subtitle: None,
            description: "To accelerate the growth of the space industry by helping to pioneer safer, faster, and better autonomous systems and to advance the state of robotics technology as it applies to space flight and exploration".to_string(),
        };

        let objective = vec![objective];

        let highschool = SectionContent {
            title: "South Brunswick High School".to_string(),
            subtitle: Some("Monmouth Junction, NJ : 2016 - 2020".to_string()),
            description: "High School Diploma, GPA: 4.5 (W) / 3.9 (UW)".to_string(),
        };
        let college = SectionContent {
            title: "Purdue University".to_string(),
            subtitle: Some("West Lafayette, IN : 2020 - May 2024".to_string()),
            description:
                "Bachelor of Science in Aeronautical and Astronautical Engineering, GPA: 4.0"
                    .to_string(),
        };

        let education = vec![college, highschool];

        let blrs_lead = SectionContent {
            title: "Purdue SigBots".to_string(),
            subtitle: Some("Software Subteam Lead : June 2021 - Present".to_string()),
            description: "Leading the development of competition robot software, teaching new members, finding ways to retain members for multiple seasons".to_string(),
        };

        let blrs_member = SectionContent {
            title: "Purdue SigBots".to_string(),
            subtitle: Some("Software Subteam Member : September 2020 - June 2021".to_string()),
            description: "Assisted in the development of a chassis control library, implemented autonomous scoring routines for competitive VEX Robotics matches".to_string(),
        };

        let lunabotics = SectionContent {
            title: "Purdue Lunabotics".to_string(),
            subtitle: Some("ROS Developer : August 2020 - June 2021".to_string()),
            description: "Researched image recognition techniques for robot localization, helped develop ROS framework and Arduino-based motor control for use in NASA Robotic Mining Competition".to_string(),
        };

        let davidoff = SectionContent {
            title: "Davidoff & Caruso CPAs, Cranbury, NJ".to_string(),
            subtitle: Some("Student Assistant : July 2017 - March 2020".to_string()),
            description: "Aided in clerical work, digitized past client files, created spreadsheets for organizing and cataloging client information".to_string(),
        };

        let experience = vec![blrs_lead, blrs_member, lunabotics, davidoff];

        let technical = SectionContent {
            title: "Technical Skills".to_string(),
            subtitle: None,
            description:
                "Proficient in Java and C++, working knowledge of Rust, Python, Javascript. Experience with Arduino, Raspberry Pi, ROS (Robotic Operating System), and CAD software (Siemens NX and Autodesk Inventor). Experience with Outlook, Excel, and PowerPoint."
                    .to_string(),
        };

        let gen_skills = SectionContent {
            title: "General Skills".to_string(),
            subtitle: None,
            description: "Teamwork, leadership, time management, teaching & communication. Able to think critically and creatively to solve problems. Strong work ethic and determination to find a robust solution to any situation".to_string(),
        };

        let skills = vec![technical, gen_skills];

        let stamps = SectionContent {
            title: "Stamps Scholarship".to_string(),
            subtitle: Some("2020 - Present".to_string()),
            description: "Nationally prestigious merit scholarship awarded to college freshmen. Received a full ride scholarship to Purdue University, as well as enrichment funds for activities including study abroad, leadership conferences, etc.".to_string(),
        };

        let achievements = vec![stamps];

        let dnd = SectionContent {
            title: "Dungeons & Dragons".to_string(),
            subtitle: Some("Player and Dungeon Master : 2020 - Present".to_string()),
            description: "I got into D&D in the summer of 2020, and I introduced it to my friends winter of that same year. We have weekly sessions where we can get together, have fun, and relax for a couple of hours. As Dungeon Master, I am able to create complex worlds and stories for my players to participate in and enjoy every Saturday.".to_string(),
        };

        let writing = SectionContent {
            title: "Writing".to_string(),
            subtitle: Some("Summer 2019 - Present".to_string()),
            description: "For the past couple of years, I have been using writing as a form of creative outlet and expression. I write about whatever is on my mind or stories that I come up with; more recently, I have started writing science fiction short stories that combine my areas of interest with larger social issues. I grew up reading a lot of Jules Verne and Isaac Asimov, and I hope to one day achieve their level of rich storytelling.".to_string(),
        };

        let hobbies = vec![dnd, writing];

        html! {
            <div class="resume">

                <div style="display: flex; justify-contents: center;">
                    <img id="headshot" src="images/headshot.jpeg"/>
                </div>

                <Section name="Objective" content={objective}/>
                <Section name="Education" content={education} />
                <Section name="Experience" content={experience} />
                <Section name="Skills" content={skills} />
                <Section name="Achievements" content={achievements} />
                <Section name="Hobbies" content={hobbies} />
            </div>
        }
    }
}

struct Section {
}

#[derive(Properties, Clone, PartialEq)]
struct SectionProps {
    name: String,
    content: Vec<SectionContent>,
}

enum SectionMsg {}

impl Component for Section {
    type Properties = SectionProps;
    type Message = SectionMsg;

    fn create(_ctx: &Context<Self>) -> Self {
        Section {
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let name = &ctx.props().name;
        let content = &ctx.props().content;
        html! {
            <div class="section">
                <h2 class="section-name">{name.as_str()}</h2>
                {content.into_iter().map(|sec| html!(
                    <div class="section-content">
                        <h2 class="section-title">{sec.title.as_str()}</h2>
                        { if sec.subtitle.is_some() { html!(
                            <h3 class="section-subtitle">{sec.subtitle.as_ref().unwrap()}</h3>
                        )} else { html!(<></>)}}
                        <p class="section-description">{sec.description.as_str()}</p>
                    </div>
                )).collect::<Html>()}
            </div>
        }
    }
}
#[derive(Clone, PartialEq)]
struct SectionContent {
    title: String,
    subtitle: Option<String>,
    description: String,
}
