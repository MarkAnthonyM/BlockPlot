use ybc::{Control, Field, Section};

use yew::prelude::*;

pub struct SignUp {
    link: ComponentLink<Self>,
    _state: State,
}

pub enum Msg {
    PostData,
}

struct State {}

impl SignUp {
    fn api_key_view(&self) -> Html {
        html! {
            <Field>
                <label class="label">{ "RescueTime Api Key" }</label>
                <Control>
                    <input
                        class="input"
                        name="api_key"
                        placeholder="Input user api key"
                    />
                </Control>
            </Field>
        }
    }

    fn user_email_view(&self) -> Html {
        html! {
            <Field>
                <label class="label">{ "User Email" }</label>
                <Control>
                    <input
                        class="input"
                        name="user_email"
                        placeholder="Input user email address"
                    />
                </Control>
            </Field>
        }
    }

    fn username_view(&self) -> Html {
        html! {
            <Field>
                <label class="label">{ "Username" }</label>
                <Control>
                    <input
                        class="input"
                        name="username"
                        placeholder="Create a Username"
                    />
                </Control>
            </Field>
        }
    }
}

impl Component for SignUp {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            link: _link,
            _state: State {},
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
                                { self.username_view() }
                                { self.user_email_view() }
                                { self.api_key_view() }
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
