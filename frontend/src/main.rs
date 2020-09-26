use yew::prelude::*;

enum Msg { }

struct Model {
    // "ComponentLink is like a reference to a component"
    _link: ComponentLink<Self>,
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
            <div>
                <h1>{ "Hello, World!" }</h1>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
