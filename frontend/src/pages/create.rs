pub struct Form {
    state: State,
    link: ComponentLink<Self>,
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

struct State {
    form_data: FormData,
    post_form_error: Option<Error>,
    post_form_loaded: bool,
}

impl Component for Form {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let mock_form_data = FormData {
            name: "Test skill",
            description: "This is a test description",
        };
        
        Self {
            state: State {
                form_data: mock_form_data,
                post_form_error: None,
                post_form_success: false,
            },
            _link,
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PostData => {
                println!("Post some data!");

                true
            },
            Msg::PostDataError => {
                println!("Error posting data!");

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
            <h1>
                <p>{ "Hello, World!" }</p>
            </h1>
        }
    }
}