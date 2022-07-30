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
        let about_me_1 = "Hello! My name is Mihir, I'm an aerospace engineering student at Purdue University, specializing in autonomy and controls. I want to help accelerate the growth of the space industry by helping to pioneer safer, faster, and better autonomous systems and to advance the state of robotics technology as it applies to space flight and exploration".to_string();

        let about_me_1 = SectionContent {
            title: "".to_string(),
            subtitle: None,
            description: about_me_1,
        };

        let about_me_2 = "I am currently looking for an internship for Summer 2023 working on autonomy in aerospace or related fields. Feel free to contact me through email or other means if you think I'd be a good fit for a role at your company! Below, you can find some of the projects I've worked on and the skills I've learned.".to_string();

        let about_me_2 = SectionContent {
            title: "".to_string(),
            subtitle: None,
            description: about_me_2,
        };

        let about_me = vec![about_me_1, about_me_2];

        let highschool = SectionContent {
            title: "South Brunswick High School".to_string(),
            subtitle: Some("High School Diploma : 2016 - 2020".to_string()),
            description: "Coursework: Data Structures & Algorithms, Mobile App Development. GPA: 4.5/4.0".to_string(),
        };
        let college = SectionContent {
            title: "Purdue University".to_string(),
            subtitle: Some("BS in Aeronautical and Astronautical Engineering : 2020 - May 2024".to_string()),
            description:
                "Coursework: Multivariable Calculus, Linear Algebra, Differential Equations and Partial Differential Equations, Aeromechanics I & II, Thermodynamics I, Signal Analysis, Intro to Aerospace Design. GPA: 4.0/4.0"
                    .to_string(),
        };

        let education = vec![college, highschool];

        let research = SectionContent {
            title: "Purdue Autonomous Intelligent Multi-Agent Systems Laboratory".to_string(),
            subtitle: Some("Undergraduate Research Assistant : September 2021 - May 2022".to_string()),
            description: "Developed ROS flight controller for Pixhawk flight computer. Implemented and tested controller on custom camera-enabled quadrotor platform. Presented weekly on research progress to graduate members of research lab.".to_string(),
        };

        let collins = SectionContent {
            title: "Collins Aerospace".to_string(),
            subtitle: Some("Software Engineering Co-op : May 2022 - Present".to_string()),
            description: "Develop test cases for verification of high-level software requirements for Flight Management Systems software. Write automated Python test scripts to execute on virtual testbed. Author peer reviews and revise tests based on feedback.".to_string(),
        };

        let experience = vec![collins, research];

        let blrs_lead = SectionContent {
            title: "Purdue SIGBots".to_string(),
            subtitle: Some("Software Subteam Lead : June 2021 - May 2022".to_string()),
            description: "Trained new software recruits on ARMS, Purdue's in-house chassis control library. Improved ARMS’s autonomous movement functionality for new AI competition team. Documented software team progress through meeting notes and club-wide presentations.".to_string(),
        };

        let blrs_member = SectionContent {
            title: "Purdue SIGBots".to_string(),
            subtitle: Some("Software Subteam Member : September 2020 - June 2021".to_string()),
            description: "Write autonomous subroutines for robots to compete in VEX Robotics Competition. Research pure-pursuit algorithms for use in robotic navigation of playing field. Created VOSS library used to provide a common codebase for programming robot mechanisms.".to_string(),
        };

        let lunabotics = SectionContent {
            title: "Purdue Lunabotics".to_string(),
            subtitle: Some("ROS Developer : August 2020 - June 2021".to_string()),
            description: "Researched image recognition techniques for robot localization. Developed ROS framework and Arduino-based motor control for use in NASA Robotic Mining Competition.".to_string(),
        };

        let extracurriculars = vec![blrs_lead, blrs_member, lunabotics];

        let lang = SectionContent {
            title: "Programming Languages".to_string(),
            subtitle: None,
            description:
                "Java (5 years), C++ (5 years), MATLAB (2 years), Python (2 years)."
                    .to_string(),
        };

        let software = SectionContent {
            title: "Software & Tools".to_string(),
            subtitle: None,
            description:
                "ROS (Robotic Operating System), CAD software (Siemens NX and Autodesk Inventor), Git, SVN, Outlook, MS Teams, Excel, and PowerPoint."
                    .to_string(),
        };

        let hardware = SectionContent {
            title: "Hardware".to_string(),
            subtitle: None,
            description: "Arduino, Raspberry Pi, Pixhawk PX4 Mini, Qualisys Motion Capture. Experience working with ultrasonic, light, and vision sensing.".to_string(),
        };

        let gen_skills = SectionContent {
            title: "General Skills".to_string(),
            subtitle: None,
            description: "Teamwork, leadership, time management, teaching & communication. Able to think critically and creatively to solve problems. Strong work ethic and determination to find a robust solution to any situation".to_string(),
        };

        let skills = vec![lang, software, hardware, gen_skills];

        html! {
            <div class="resume">

                <Section name="About Me" content={about_me}/>
                <Section name="Education" content={education} />
                <Section name="Experience" content={experience} />
                <Section name="Extracurriculars" content={extracurriculars} />
                <Section name="Skills" content={skills} />
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
                <div class="section-content">
                    <h2 class="section-name">{name.as_str()}</h2>
                    {content.into_iter().map(|sec| html!(
                        <div class="section-inner-content">
                            <h2 class="section-title">{sec.title.as_str()}</h2>
                            { if sec.subtitle.is_some() { html!(
                                <h3 class="section-subtitle">{sec.subtitle.as_ref().unwrap()}</h3>
                            )} else { html!(<></>)}}
                            <p class="section-description">{sec.description.as_str()}</p>
                        </div>
                    )).collect::<Html>()}
                </div>
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
