use anyhow::Error;

use chrono::prelude::*;
use chrono::Duration;
use yew_router::agent::RouteRequest;
use yew_router::prelude::*;

use crate::api;
use crate::route::Route::UnauthorizedPage;
use crate::types::{Color, TimeData, TimeStats, TimeWrapper};

use ybc::TileCtx::{Ancestor, Child, Parent};
use ybc::TileSize;
use ybc::{Box, Container, Section, Tile};

use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchTask, StatusCode};
use yew::services::ConsoleService;

pub enum Msg {
    GetDevSkillBlock,
    GetSkillBlocksSuccess(TimeWrapper),
    GetSkillBlocksError(Error),
    UnauthorizedAccess,
}

pub struct User {
    state: State,
    link: ComponentLink<Self>,
    router: RouteAgentDispatcher<()>,
    task: Option<FetchTask>,
}

struct State {
    skill_blocks: Vec<TimeData>,
    get_skillblocks_error: Option<Error>,
    get_skillblocks_loaded: bool,
}

impl User {
    // Create calender grid element
    fn view_blockgrid(&self, time_block: &TimeData) -> Html {
        // Create empty vecotr representing months out of a year
        let mut month_elements = Vec::new();

        // create empty vector representing weeks out of a year
        let mut week_elements = Vec::new();

        // create empty vector representing days of a week
        let mut day_elements = Vec::new();

        let mut time_stats = TimeStats {
            daily_max: 0,
            yearly_max: 0,
            longest_chain: 0,
        };

        // Count of consective days having time data
        let mut chain_count = 0;

        // Create vector of timestamps for one year
        let current_date = Local::now().date().naive_utc();
        let (current_year, current_month, current_day) = (
            current_date.year(),
            current_date.month(),
            current_date.day(),
        );

        // Calculate value to subtract from current day. When new week starts on a sunday,
        // 0 is subtracted from current day, shifting calender graph leftward and replacing oldest week
        // TODO: Fix bug when week day is sunday. overflow issue.
        let day_incrementor = current_date.weekday().pred().num_days_from_monday();
        let week_incrementor = day_incrementor % 6;
        let oldest_week = current_day - week_incrementor;

        let year_start = NaiveDateTime::new(
            NaiveDate::from_ymd(current_year - 1, current_month, oldest_week),
            NaiveTime::from_hms(0, 0, 0),
        );
        let year_end = NaiveDateTime::new(
            NaiveDate::from_ymd(current_year, current_month, current_day),
            NaiveTime::from_hms(0, 0, 0),
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

                    // Create month <text> elements, spaced out by first sunday of every month
                    let month = day.format("%h");
                    // Calculate week number of given month
                    let week_number = (day.day() - 1) / 7 + 1;
                    // Check if date is first week of given month
                    if week_number == 1 {
                        let month_element = html! {
                            <text class="month" y="-7" x=format!("{}", week_elements.len() * 14) style="font-size: 12px;">{ month }</text>
                        };

                        month_elements.push(month_element);
                    }
                }
            }

            if let Some(value) = time_block.time_data.get(&day) {
                let minutes = value / 60;

                // Time statistics calculations
                if minutes > time_stats.daily_max {
                    time_stats.daily_max = minutes;
                }
                time_stats.yearly_max += minutes;

                // Represent time data as color
                match minutes {
                    0 => color = Color::NEUTRAL,
                    1..=15 => color = Color::LIGHT,
                    16..=30 => color = Color::LIGHTMEDIUM,
                    31..=45 => color = Color::MEDIUM,
                    46..=60 => color = Color::MEDIUMHIGH,
                    _ => color = Color::HIGH,
                }

                // Calculate length of longest chain of days that have time data
                match color {
                    Color::NEUTRAL => {
                        if chain_count != 0 {
                            if chain_count > time_stats.longest_chain {
                                time_stats.longest_chain = chain_count;
                                chain_count = 0;
                            } else {
                                chain_count = 0;
                            }
                        }
                    }
                    _ => {
                        chain_count += 1;
                    }
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

        // Time stats for hours/minutes labels
        let year_minutes_average = time_stats.yearly_max / 365;
        let year_hours_average = year_minutes_average / 60;
        let year_minutes_remainder = year_minutes_average % 60;
        let max_hours = time_stats.daily_max / 60;
        let max_minutes = time_stats.daily_max % 60;

        // Create svg container, collect grid elements and append to <g> tag. Append week/month labels
        let html_element = html! {
            <>
                <Box>
                    <svg width="780" height="128">
                        <g transform="translate(30, 20)">
                            { week_elements.into_iter().collect::<Html>() }
                            { month_elements.into_iter().collect::<Html>() }
                            <text text-anchor="start" class="wday" dx="-30" dy="8" style="display: none; font-size: 12px;">{ "Sun" }</text>
                            <text text-anchor="start" class="wday" dx="-30" dy="25" style="font-size: 12px;">{ "Mon" }</text>
                            <text text-anchor="start" class="wday" dx="-30" dy="32" style="display: none; font-size: 12px;">{ "Tue" }</text>
                            <text text-anchor="start" class="wday" dx="-30" dy="56" style="font-size: 12px;">{ "Wed" }</text>
                            <text text-anchor="start" class="wday" dx="-30" dy="57" style="display: none; font-size: 12px;">{ "Thurs" }</text>
                            <text text-anchor="start" class="wday" dx="-30" dy="85" style="font-size: 12px;">{ "Fri" }</text>
                            <text text-anchor="start" class="wday" dx="-30" dy="81" style="display: none; font-size: 12px;">{ "Sat" }</text>
                        </g>
                    </svg>
                </Box>
                <nav class="level">
                    <div class="level-item has-text-centered">
                        <div>
                            <p class="heading">{ "Average Daily Time" }</p>
                            <p class="title">{ year_hours_average }</p>
                            <p class="heading">{"hour(s)"}</p>
                            <p class="title">{ year_minutes_remainder }</p>
                            <p class="heading">{ "minute(s)" }</p>
                        </div>
                    </div>
                    <div class="level-item has-text-centered">
                        <div>
                            <p class="heading">{ "Max Time In A Day" }</p>
                            <p class="title">{ max_hours }</p>
                            <p class="heading">{"hour(s)"}</p>
                            <p class="title">{ max_minutes }</p>
                            <p class="heading">{ "minute(s)" }</p>
                        </div>
                    </div>
                    <div class="level-item has-text-centered">
                        <div>
                            <p class="heading">{ "Longest Day Chain" }</p>
                            <p class="title">{ time_stats.longest_chain }</p>
                            <p class="heading">{ "day(s)" }</p>
                            <p class="title">{ chain_count }</p>
                            <p class="heading">{ "current day chain" }</p>
                        </div>
                    </div>
                </nav>
            </>
        };

        html_element
    }

    // Create skill block item.
    fn view_skill_blocks(&self) -> Html {
        let mut block_elements = Vec::new();

        for block in &self.state.skill_blocks {
            let block_element = html! {
                <Tile ctx=Ancestor>
                    <Tile ctx=Parent size=TileSize::Four>
                        <Tile classes=Some("notification is-primary") ctx=Child>
                            <p class="title is-3">{ "Skill:" }</p>
                            <p class="subtitle is-5">{ &block.skill_name }</p>
                            <p class="title is-3">{ "Category:" }</p>
                            <p class="subtitle is-5">{ &block.category }</p>
                            <p class="title is-3">{ "Description:" }</p>
                            <p class="subtitle is-5">{ &block.skill_description }</p>
                        </Tile>
                    </Tile>
                    <Tile ctx=Parent size=TileSize::Eight>
                        <Tile classes=Some("notification is-primary") ctx=Child>
                            //TODO: Fix overflow issue
                            { self.view_blockgrid(block) }
                        </Tile>
                    </Tile>
                </Tile>
            };

            block_elements.push(block_element);
        }

        html! {
            <Container>
                { block_elements.into_iter().collect::<Html>() }
            </Container>
        }
    }
}

impl Component for User {
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
            router: RouteAgentDispatcher::new(),
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            // Testing logic, may remove
            Msg::GetDevSkillBlock => {
                self.state.get_skillblocks_loaded = false;
                let handler =
                    self.link
                        .callback(move |response: api::FetchResponse<TimeWrapper>| {
                            let (meta, Json(data)) = response.into_parts();
                            // Check for 401, redirect to UnauthorizedPage route if true
                            if meta.status == StatusCode::UNAUTHORIZED {
                                Msg::UnauthorizedAccess
                            } else {
                                match data {
                                    Ok(skillblocks) => Msg::GetSkillBlocksSuccess(skillblocks),
                                    Err(error) => Msg::GetSkillBlocksError(error),
                                }
                            }
                        });
                self.task = Some(api::get_dev_skillblocks(handler));
                true
            }
            Msg::GetSkillBlocksError(error) => {
                self.state.get_skillblocks_error = Some(error);
                self.state.get_skillblocks_loaded = true;
                true
            }
            Msg::GetSkillBlocksSuccess(skillblocks) => {
                for skillblock in skillblocks.data {
                    self.state.skill_blocks.push(skillblock);
                    self.state.get_skillblocks_loaded = true;
                }

                true
            }
            Msg::UnauthorizedAccess => {
                //TODO: Implement logic to destory session state
                // stored on frontend
                let route = RouteRequest::ChangeRoute(Route::from(UnauthorizedPage));
                self.router.send(route);

                true
            }
        }
    }

    fn change(&mut self, _prop: Self::Properties) -> ShouldRender {
        // Should only return "true" if new porperties are different to previously recieved properties.
        false
    }

    fn view(&self) -> Html {
        html! {
            <Section>
                { self.view_skill_blocks() }
            </Section>
        }
    }
}
