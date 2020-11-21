use ybc::{ Control, Field, Section };

use yew::prelude::*;

pub struct SignIn {
    link: ComponentLink<Self>,
    _state: State,
}

pub enum Msg {
    PostData,
}

pub struct State {}

impl Component for SignIn {
    type Message = Msg;
    type Properties = ();
    
    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            link: _link,
            _state: State {

            }
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <main class="bd-main">
                <Section>
                    <div class="colums">
                        <div class="column is-half">
                            <form action="placeholder/replace" method="POST">
                                <Field>
                                    <div class="control">
                                        <button
                                            class="button is-link"
                                            onsubmit=self.link.callback(|_| Msg::PostData)>{ "Submit" }
                                        </button>
                                    </div>
                                </Field>
                            </form>
                        </div>
                    </div>
                </Section>
            </main>
        }
    }
}