use anyhow::Error;

use crate::types::FormData;

use ybc::{ Control, Field, Section };

use yew::prelude::*;
use yew::format::Json;
use yew::services::console::ConsoleService;
use yew::services::fetch::{ FetchService, FetchTask, Request, Response };

pub struct Form {
    link: ComponentLink<Self>,
    props: Props,
    state: State,
    task: Option<FetchTask>,
}

pub enum Msg {
    Ignore,
    PostData,
    PostDataSuccess,
    PostDataError(Error),
    SetCategory(String),
    SetDescription(String),
    SetSkillName(String),
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
            category: "".to_string(),
            description: "This is a test description".to_string(),
            skill_name: "".to_string(),
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
                // let text = std::mem::replace(&mut self.state.form_data.skill_name, self.props.value.clone());
                // self.props.onsubmit.emit(text);
                //TODO: Prototype method to submit form data. Try to find better method 
                let handler =
                    self.link
                        .callback(move |response: Response<Result<String, Error>>| {
                            let (_, data) = response.into_parts();
                            match data {
                                Ok(value) => {
                                    ConsoleService::log(&value);
                                },
                                Err(error) => {
                                    ConsoleService::log("Error with transmission");
                                }
                            }
                            Msg::Ignore
                        });
                
                let form_instance = FormData {
                    category: self.state.form_data.category.clone(),
                    description: self.state.form_data.description.clone(),
                    skill_name: self.state.form_data.skill_name.clone(),
                };
                
                let url = format!("http://localhost:8000/api/testpost");
                let request = Request::post(url)
                    .header("Content-Type", "application/json")
                    .body(Json(&form_instance))
                    .unwrap();
                
                self.task = FetchService::fetch(request, handler).ok();

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
            Msg::SetCategory(text) => {
                self.state.form_data.category = text;
                let current_text = format!("{}", self.state.form_data.category);
                ConsoleService::log(&current_text);

                true
            },
            Msg::SetDescription(text) => {
                self.state.form_data.description = text;
                let current_text = format!("{}", self.state.form_data.description);
                ConsoleService::log(&current_text);

                true
            }
            Msg::SetSkillName(text) => {
                self.state.form_data.skill_name = text;
                let current_text = format!("{}", self.state.form_data.skill_name);
                ConsoleService::log(&current_text);

                true
            },
            Msg::Ignore => { true }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            self.state.form_data.skill_name = self.props.value.clone();
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let onselect = self.link.callback(|event| match event {
            ChangeData::Select(elem) => {
                let value = elem.value();

                Msg::SetCategory(value)
            },
            _ => unreachable!(),
        });
        
        html! {
            <>
                <main class="bd-main">
                    <Section>
                        <div class="colums">
                            <div class="column is-half">
                                <form>
                                    <Field>
                                        <label class="label">{ "Skill Name" }</label>
                                        <Control>
                                            <input
                                                class="input"
                                                placeholder="Text input"
                                                oninput=self.link.callback(|e: InputData| Msg::SetSkillName(e.value))
                                            />
                                        </Control>
                                    </Field>
                                    <Field>
                                        <label class="label">{ "Skill Category" }</label>
                                        <Control>
                                            <div class="select">
                                                <select onchange=onselect>
                                                    <option value=0>{ "Software Development" }</option>
                                                    <option value=1>{ "References & Learning" }</option>
                                                </select>
                                            </div>
                                        </Control>
                                    </Field>
                                    <Field>
                                        <label class="label">{ "Skill Description" }</label>
                                        <Control>
                                            <input
                                                class="input"
                                                placeholder="Skill Description"
                                                oninput=self.link.callback(|e: InputData| Msg::SetDescription(e.value))
                                            />
                                        </Control>
                                    </Field>
                                    <Field>
                                        <div class="control">
                                            <button
                                                class="button is-link"
                                                onclick=self.link.callback(|_| Msg::PostData)>{ "Submit" }
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