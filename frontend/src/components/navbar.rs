use ybc::Section;

use yew::prelude::*;

pub struct Navbar {}

impl Component for Navbar {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div class="container">
                    { self.view_navbar() }
                </div>
                <Section>
                    { self.view_skill_blocks() }
                </Section>
            </>
        }
    }
}