use anyhow::Error;

use ybc::{ Control, Field, Section };

use yew::prelude::*;

pub struct Form {
    link: ComponentLink<Self>,
    props: Props,
    state: State,
}

pub enum Msg {
    Ignore,
    PostData,
    PostDataSuccess,
    PostDataError(Error),
    ToggleCategory,
}

#[derive(Properties, Clone, Default, PartialEq)]
pub struct Props {
    pub value: String,
    pub onsubmit: Callback<String>,
}

struct State {
    post_form_error: Option<Error>,
    post_form_loaded: bool,
    toggle_category: bool,
}

impl Form {
    fn offline_category_view(&self) -> Html {
        html! {
            <Field>
            <Control>
                <label class="checkbox">
                    <input
                        type="checkbox"
                        name="offline_category"
                    />
                    { "Category Offline?" }
                </label>
                </Control>
            </Field>
        }
    }
    
    fn skill_category_view(&self) -> Html {
        if !self.state.toggle_category {
            html! {
                <Field>
                    <label class="label">{ "Skill Category" }</label>
                    <Control>
                        <div class="select">
                            <select required=true name="category">
                                <option value="" disabled=true selected=true hidden=true>{ "Selected Category" }</option>
                                <option value="software development">{ "Software Development" }</option>
                                <option value="references & learning">{ "References & Learning" }</option>
                            </select>
                        </div>
                    </Control>
                </Field>
            }
        } else {
            html! {
                <Field>
                    <label class="label">{ "Offline Category Name" }</label>
                    <Control>
                        <input
                            class="input"
                            name="offline_category"
                            placeholder="Offline Category Input"
                        />
                    </Control>
                </Field>
            }
        }
    }

    fn skill_description_view(&self) -> Html {
        html! {
            <Field>
                <label class="label">{ "Skill Description" }</label>
                <Control>
                    <input
                        class="input"
                        name="description"
                        placeholder="Skill Description"
                    />
                </Control>
            </Field>
        }
    }

    fn skill_name_view(&self) -> Html {
        html! {
            <Field>
                <label class="label">{ "Skill Name" }</label>
                <Control>
                    <input
                        class="input"
                        name="skill_name"
                        placeholder="Text input"
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
                        placeholder="Username Input"
                    />
                </Control>
            </Field>
        }
    }
}

impl Component for Form {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            link: _link,
            state: State {
                post_form_error: None,
                post_form_loaded: false,
                toggle_category: false,
            },
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PostData => {
                println!("Data posted!");

                true
            },
            Msg::PostDataError(error) => {
                println!("Error posting data: {:?}", error);

                true
            },
            Msg::PostDataSuccess => {
                println!("Success posting data!");

                true
            },
            Msg::ToggleCategory => {
                self.state.toggle_category = !self.state.toggle_category;

                true
            },
            Msg::Ignore => { true }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                <main class="bd-main">
                    <Section>
                        <div class="colums">
                            <div class="column is-half">
                                <form action="http://localhost:8000/api/testpost" method="POST">
                                    { self.username_view() }
                                    { self.skill_name_view() }
                                    { self.offline_category_view() }
                                    { self.skill_category_view() }
                                    { self.skill_description_view() }
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
            </>
        }
    }
}