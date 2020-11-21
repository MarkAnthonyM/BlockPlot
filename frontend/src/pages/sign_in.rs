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

impl SignIn {
    fn username_view(&self) -> Html {
        html! {
            <Field>
                <label class="label">{ "Username" }</label>
                <Control>
                    <input
                        class="input"
                        name="username"
                        placeholder="Input username"
                    />
                </Control>
            </Field>
        }
    }

    fn password_view(&self) -> Html {
        html! {
            <Field>
                <label class="label">{ "Password" }</label>
                <Control>
                    <input
                        class="input"
                        name="password"
                        placeholder="Input password"
                    />
                </Control>
            </Field>
        }
    }
}

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
                                { self.username_view() }
                                { self.password_view() }
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