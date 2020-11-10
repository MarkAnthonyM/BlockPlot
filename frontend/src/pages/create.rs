use anyhow::Error;

use ybc::{ Control, Field, Section };

use yew::prelude::*;
use yew::services::fetch::FetchTask;

pub struct Form {
    link: ComponentLink<Self>,
    props: Props,
    state: State,
    task: Option<FetchTask>,
}

// Might not be used. Seems similar to SkillBlock struct
struct FormData {
    category: String,
    description: String,
    skill_name: String,
}

pub enum Msg {
    PostData,
    PostDataSuccess,
    PostDataError(Error),
}

#[derive(Properties, Clone, Default, PartialEq)]
pub struct Props {
    pub value: String,
    pub onsubmit: Callback<String>,
}

struct State {
    form_data: FormData,
    post_form_error: Option<Error>,
    post_form_loaded: bool,
}

impl Component for Form {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let mock_form_data = FormData {
            category: "This is a test category".to_string(),
            description: "This is a test description".to_string(),
            skill_name: "Test skill".to_string(),
        };
        
        Self {
            link: _link,
            state: State {
                form_data: mock_form_data,
                post_form_error: None,
                post_form_loaded: false,
            },
            props,
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PostData => {
                println!("Post some data!");

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
        }
    }

    fn change(&mut self, _prop: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <main class="bd-main">
                    <Section>
                        <div class="colums">
                            <div class="column is-half">
                                <Field>
                                    <label class="label">{ "Skill Name" }</label>
                                    <Control>
                                        <input class="input" placeholder="Text input"/>
                                    </Control>
                                </Field>
                                <Field>
                                    <label class="label">{ "Category" }</label>
                                    <Control>
                                        <div class="select">
                                            <select>
                                                <option>{ "Software Development" }</option>
                                                <option>{ "References & Learning" }</option>
                                            </select>
                                        </div>
                                    </Control>
                                </Field>
                                <Field>
                                    <label class="label">{ "Description" }</label>
                                    <Control>
                                        <input class="input" placeholder="Skill Description"/>
                                    </Control>
                                </Field>
                            </div>
                        </div>
                    </Section>
                </main>
            </>
        }
    }
}