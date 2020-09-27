#![recursion_limit = "256"]

use ybc::{ Box, Container, Navbar, Section, Tile };
use ybc::TileCtx::{ Ancestor, Child, Parent };
use ybc::TileSize;

use yew::prelude::*;

enum Msg { }

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

    fn view_navbar(&self) -> Html {
        html! {
            <Navbar navbrand=self.view_navbrand() navstart=self.view_navstart() navend=self.view_navend() />
        }
    }

    fn view_navbrand(&self) -> Html {
        html! {

        }
    }

    fn view_navend(&self) -> Html {
        html! {

        }
    }

    fn view_navstart(&self) -> Html {
        html! {

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
