use ybc::{ Control, Field, Section };

use yew::prelude::*;

pub struct Form {
    link: ComponentLink<Self>,
    props: Props,
    state: State,
}

pub enum Msg {
    PostData,
    ToggleCategory,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub key_present: bool,
}

struct State {
    toggle_category: bool,
}

impl Form {
    fn api_key_view(&self) -> Html {
        if !self.props.key_present {
            html! {
                <Field>
                    <p>
                        {
                            "Hey! Looks like we don't have your RescueTime API key on record. We're gonna need that.
                            Grab it or create one from your RescueTime API key management page, toss it in below, and away we'll go!"
                        }
                    </p>
                    <label class="label">{ "RescueTime Api Key" }</label>
                    <Control>
                        <input
                            class="input"
                            name="api_key"
                            placeholder="Text input"
                        />
                    </Control>
                </Field>
            }
        } else {
            html! {
                <>
                </>
            }
        }
    }
    
    fn offline_category_view(&self) -> Html {
        html! {
            <Field>
            <Control>
                <label class="checkbox">
                    <input
                        type="checkbox"
                        name="offline_category"
                        onclick=self.link.callback(|_| Msg::ToggleCategory)
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
                            name="category"
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
}

impl Component for Form {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            link: _link,
            props,
            state: State {
                toggle_category: false,
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PostData => {
                println!("Data posted!");

                true
            },
            Msg::ToggleCategory => {
                self.state.toggle_category = !self.state.toggle_category;

                true
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <main class="bd-main">
                    <Section>
                        <div class="colums">
                            <div class="column is-half">
                                <form action="http://localhost:8000/api/testpost" method="POST">
                                    { self.api_key_view() }
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