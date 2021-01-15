use anyhow::Error;

use crate::api;
use crate::components::NavbarElement;
use crate::pages::{ About, Form, Home, SignIn, SignUp, Unauthorized, User };
use crate::route::Route;
use crate::types::Session;

use yew::format::Json;
use yew::prelude::*;
use yew_router::prelude::*;
use yew::services::fetch::FetchTask;


pub enum Msg {
    CheckLoginStatus,
    GetSessionError(Error),
    GetSessionSuccess(Session),
}

pub struct App {
    link: ComponentLink<Self>,
    state: State,
    task: Option<FetchTask>,
}

pub struct State {
    session: Option<Session>,
    session_checked: bool,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::CheckLoginStatus);
        
        Self {
            link,
            state: State {
                session: None,
                session_checked: false,
            },
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CheckLoginStatus => {
                self.state.session_checked = false;
                let handler =
                    self.link
                        .callback(move |response: api::FetchResponse<Session>| {
                            let (_, Json(data)) = response.into_parts();
                            match data {
                                Ok(session_info) => Msg::GetSessionSuccess(session_info),
                                Err(error) => Msg::GetSessionError(error),
                            }
                        });
                self.task = Some(api::get_user_session(handler));
                true
            },
            Msg::GetSessionError(_error) => {
                self.state.session = None;
                self.state.session_checked = true;
                true
            },
            Msg::GetSessionSuccess(session) => {
                self.state.session = Some(session);
                self.state.session_checked = true;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let navbar_div;
        let key_exists;
        match self.state.session.as_ref() {
            Some(session) => {
                key_exists = session.key_present;
                navbar_div = html! {
                    <NavbarElement session=session/>
                }
            },
            None => {
                key_exists = false;
                navbar_div = html! {
                    <NavbarElement session=None/>
                }
            }
        };

        let render = Router::render(move |switch: Route| match switch {
            Route::AboutPage => html! {<About/>},
            Route::FormPage => html! {<Form key_present=key_exists/>},
            Route::SignInPage => html! {<SignIn/>},
            Route::SignUpPage => html! {<SignUp/>},
            Route::UnauthorizedPage => html! { <Unauthorized/> },
            Route::UserPage => html! {<User/>},
            Route::HomePage => html! {<Home/>},
        });

        html! {
            <>
                { navbar_div }
                <Router<Route, ()> render=render/>
            </>
        }
    }
}