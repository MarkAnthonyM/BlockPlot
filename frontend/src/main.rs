#![recursion_limit = "256"]

mod types;
mod api;

use types::{ Color, TimeData, SkillBlock };

use anyhow::Error;

use chrono::prelude::*;
use chrono::Duration;

use ybc::{ Box, Container, Navbar, NavbarItem, Section, Tile };
use ybc::NavbarItemTag::A;
use ybc::TileCtx::{ Ancestor, Child, Parent };
use ybc::TileSize;

use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

enum Msg {
    GetSkillBlocks,
    GetDevSkillBlock,
    GetSkillBlocksSuccess(TimeData),
    GetSkillBlocksError(Error),
}

struct Model {
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

struct State {
    skill_blocks: Vec<SkillBlock>,
    get_skillblocks_error: Option<Error>,
    get_skillblocks_loaded: bool,
}

impl Model {
    // Create calender grid element
    fn view_blockgrid(&self) -> Html {
        let mut html_element = html! {
            <>
            </>
        };

        for block in &self.state.skill_blocks {
            // create empty vector representing weeks out of a year
            let mut week_elements = Vec::new();
    
            // create empty vector representing days of a week
            let mut day_elements = Vec::new();
    
            // Create vector of timestamps for one year
            let current_date = Local::now().date().naive_utc();
            let (current_year, current_month, current_day) = (
                current_date.year(),
                current_date.month(),
                current_date.day()
            );
            let year_start = NaiveDateTime::new(
                NaiveDate::from_ymd(current_year - 1, current_month, current_day + 1),
                NaiveTime::from_hms(0, 0, 0)
            );
            let year_end = NaiveDateTime::new(
                NaiveDate::from_ymd(current_year, current_month, current_day),
                NaiveTime::from_hms(0, 0, 0)
            );
            let mut selected_day = year_start;
            let mut year = Vec::new();
            while selected_day <= year_end {
                year.push(selected_day);
                selected_day = selected_day + Duration::days(1);
            }
    
            // Iterate through vector of timestamps and build grid item
            for day in &year {
                let mut color = Color::NEUTRAL;
                let weekday = day.weekday();
                let formatted_date = day.format("%Y-%m-%d");
                if weekday == Weekday::Sun {
                    if day_elements.len() != 0 {
                        // Create <g> element representing a week
                        let week_element = html! {
                            <g transform=format!("translate({}, 0)", week_elements.len() * 14)>
                                { day_elements.into_iter().collect::<Html>() }
                            </g>
                        };
                        week_elements.push(week_element);
        
                        day_elements = Vec::new();
                    }
                }

                if let Some(value) = block.recent_time_data.time_data.get(&day) {
                    let minutes = value / 60;
                    match minutes {
                        0 => color = Color::NEUTRAL,
                        1..=15 => color = Color::LIGHT,
                        16..=30 => color = Color::LIGHTMEDIUM,
                        31..=45 => color = Color::MEDIUM,
                        46..=60 => color = Color::MEDIUMHIGH,
                        _ => color = Color::HIGH,
                    }
                }
                
                // Create <rect> element representing a day
                let day_element = html! {
                    <rect width="11" height="11" y=weekday.num_days_from_sunday() * 15 rx=2 ry=2 fill=color style="outline: 1px solid #1b1f230a; outline-offset: -1px;" date-data=formatted_date></rect>
                };
                day_elements.push(day_element);

                // Create tags for data of most recent weekdays 
                if day == year.last().unwrap() {
                    let week_element = html! {
                        <g transform=format!("translate({}, 0)", week_elements.len() * 14)>
                            { day_elements.into_iter().collect::<Html>() }
                        </g>
                    };

                    week_elements.push(week_element);

                    day_elements = Vec::new();
                }
            }
    
            // Create svg container, collect grid elements and append to <g> tag
            html_element = html! {
                <svg width="780" height="128">
                    <g transform="translate(20, 20)">
                        { week_elements.into_iter().collect::<Html>() }
                    </g>
                </svg>
            };
        }

        html_element
    }

    // Contruct navbar at top of page
    fn view_navbar(&self) -> Html {
        html! {
            <Navbar navbrand=self.view_navbrand() navstart=self.view_navstart() navend=self.view_navend() />
        }
    }

    // Construct navbrand section of navbar
    fn view_navbrand(&self) -> Html {
        html! {
            <NavbarItem tag=A>
                <img src="https://bulma.io/images/bulma-logo.png" />
            </NavbarItem>
        }
    }

    // Construct navend section of navbar
    fn view_navend(&self) -> Html {
        html! {

        }
    }

    // Construct main section of navbar
    fn view_navstart(&self) -> Html {
        html! {
            <>
                <NavbarItem tag=A>
                    { "Login" }
                </NavbarItem>
                <NavbarItem tag=A>
                    { "Documention" }
                </NavbarItem>
                <NavbarItem tag=A>
                    { "About" }
                </NavbarItem>
            </>
        }
    }

    // Create skill block item.
    fn view_skill_block(&self) -> Html {
        html! {
            <>
                <Tile ctx=Ancestor>
                    <Tile ctx=Parent size=TileSize::Two>
                        <Tile classes=Some("notification is-primary") ctx=Child>
                            <p class="title">{ "Example" }</p>
                        </Tile>
                    </Tile>
                    <Tile ctx=Parent size=TileSize::Eight>
                        <Tile classes=Some("notification is-primary") ctx=Child>
                            //TODO: Fix overflow issue
                            <Box>
                                { self.view_blockgrid() }
                            </Box>
                        </Tile>
                    </Tile>
                    <Tile ctx=Parent size=TileSize::Two>
                        <Tile classes=Some("notification is-primary") ctx=Child>
                            <p class="title">{ "Example" }</p>
                        </Tile>
                    </Tile>
                </Tile>
            </>
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let skill_blocks = vec![];

        link.send_message(Msg::GetDevSkillBlock);
        
        Self {
            state: State {
                skill_blocks,
                get_skillblocks_error: None,
                get_skillblocks_loaded: false,
            },
            link,
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetSkillBlocks => {
                self.state.get_skillblocks_loaded = false;
                let handler =
                    self.link
                        .callback(move |response: api::FetchResponse<TimeData>| {
                            let (_, Json(data)) = response.into_parts();
                            match data {
                                Ok(skillblocks) => Msg::GetSkillBlocksSuccess(skillblocks),
                                Err(error) => Msg::GetSkillBlocksError(error),
                            }
                        });
                self.task = Some(api::get_skillblocks(handler));
                true
            },
            Msg::GetDevSkillBlock => {
                self.state.get_skillblocks_loaded = false;
                let handler =
                    self.link
                        .callback(move |response: api::FetchResponse<TimeData>| {
                            let (_, Json(data)) = response.into_parts();
                            match data {
                                Ok(skillblocks) => Msg::GetSkillBlocksSuccess(skillblocks),
                                Err(error) => Msg::GetSkillBlocksError(error),
                            }
                        });
                self.task = Some(api::get_dev_skillblocks(handler));
                true
            },
            Msg::GetSkillBlocksError(error) => {
                self.state.get_skillblocks_error = Some(error);
                self.state.get_skillblocks_loaded = true;
                true
            },
            Msg::GetSkillBlocksSuccess(skillblocks) => {
                let skill_block = SkillBlock {
                    category: String::from("This is a test category"),
                    description: String::from("This is a test description"),
                    name: String::from("This is a test name"),
                    recent_time_data: skillblocks,
                    block_color_lite: String::from("This is a test color"),
                    block_color_regular: String::from("This is a test color"),
                    block_color_deep: String::from("This is a test color"),
                };

                self.state.skill_blocks = vec![skill_block];
                self.state.get_skillblocks_loaded = true;
                true
            },
        }
    }

    fn change(&mut self, _prop: Self::Properties) -> ShouldRender {
        // Should only return "true" if new porperties are different to previously recieved properties.
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                { self.view_navbar() }
                <Section>
                    <Container>
                        { self.view_skill_block() }
                    </Container>
                </Section>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
