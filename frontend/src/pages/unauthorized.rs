use yew::prelude::*;

pub struct Unauthorized {}

impl Component for Unauthorized {
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
                <h1>{ "Unauthorized access" }</h1>
                <p>{ "Session may have expired. Please log in and try again." }</p>
            </>
        }
    }
}