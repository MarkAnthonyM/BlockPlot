use ybc::Container;

use yew::prelude::*;

pub struct SignUp {}

impl Component for SignUp {
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
            <section class="hero is-medium is-primary">
                <div class="hero-body">
                    <Container>
                        <h1 class="title is-1">{ "Coming Soon!" }</h1>
                    </Container>
                </div>
            </section>
        }
    }
}