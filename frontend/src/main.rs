#![recursion_limit = "256"]

use ybc::{ Box, Container, Section, Tile };
use ybc::TileCtx::{ Ancestor, Child, Parent };
use ybc::TileSize;

use yew::prelude::*;

enum Msg { }

struct Model {
    // "ComponentLink is like a reference to a component"
    _link: ComponentLink<Self>,
}

impl Model {
    // Create skill block item.
    fn view_skill_block(&self) -> Html {
        html! {
            <>
                <Tile>
                    <Tile ctx=Parent size=TileSize::Two>
                        <Tile classes=Some("notification is-primary") ctx=Child>
                            <p class="title">{ "Example" }</p>
                        </Tile>
                    </Tile>
                    <Tile ctx=Parent size=TileSize::Eight>
                        <Tile classes=Some("notification is-primary") ctx=Child>
                            <Box>
                                {self.view_blockgrid()}
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
