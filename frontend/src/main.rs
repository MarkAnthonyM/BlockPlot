#![recursion_limit = "256"]

mod types;

use types::AnalyticData;

use anyhow::Error;

use ybc::{ Box, Container, Navbar, NavbarItem, Section, Tile };
use ybc::NavbarItemTag::A;
use ybc::TileCtx::{ Ancestor, Child, Parent };
use ybc::TileSize;

use yew::callback::Callback;
use yew::format::{ Nothing, Json};
use yew::prelude::*;
use yew::services::fetch::{ FetchService, FetchTask, Request, Response };

enum Msg {
    GetTimesheets,
    GetTimesheetsSuccess(Vec<AnalyticData>),
    GetTimesheetsError(Error),
}

struct Model {
    // "ComponentLink is like a reference to a component"
    _link: ComponentLink<Self>,
}

impl Model {
    // Create calender grid element
    fn view_blockgrid(&self) -> Html {
        // create empty vector representing weeks out of a year
        let mut week_elements = Vec::new();
        
        // Loop for every week in one year
        for x in 0..52 {
            // Create empty vector of blocks representing days of a week
            let mut day_elements = Vec::new();

            // Loop for everyday in one week
            for y in 0..7 {
                // Create <rect> element representing a day
                let element = html! {
                    <rect width="11" height="11" y=y * 15 rx=2 ry=2 fill="#dadada" style="outline: 1px solid #1b1f230a; outline-offset: -1px;"></rect>
                };
                
                day_elements.push(element);
            }

            // Create <g> element representing a week
            let element = html! {
                <g transform=format!("translate({}, 0)", x * 14)>
                    { day_elements.into_iter().collect::<Html>() }
                </g>
            };

            week_elements.push(element)
        }

        // Create svg container, collect grid elements and append to <g> tag
        html! {
            <svg width="750" height="128">
                <g transform="translate(20, 20)">
                    { week_elements.into_iter().collect::<Html>() }
                </g>
            </svg>
        }
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

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            _link,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
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

//TODO: Figure out the proper way to make get requests
fn api_test(callback: Callback<Response<Json<Result<T, Error>>>>) -> FetchTask {
    let request = Request::get("http://localhost:8000/times").body(Nothing).unwrap();

    FetchService::fetch(request, callback).unwrap()
}

fn main() {
    yew::start_app::<Model>();
}
