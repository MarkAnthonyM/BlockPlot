use crate::route::Route;
use crate::types::Session;

use ybc::{ Container, Navbar };

use yew::prelude::*;

use yew_router::components::RouterAnchor;

pub struct NavbarElement {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub session: Option<Session>,
}

impl NavbarElement {
    // Contruct navbar at top of page
    fn view_navbar(&self) -> Html {
        html! {
            <Navbar navbrand=self.view_navbrand() navstart=self.view_navstart() navend=self.view_navend() />
        }
    }

    // Construct navbrand section of navbar
    fn view_navbrand(&self) -> Html {
        type Anchor = RouterAnchor<Route>;
        
        html! {
            <Anchor route=Route::HomePage classes="navbar-item">
                <svg width="100" height="100">
                    <image href="static/images/logo_rough.svg" width="100" height="100"/>
                </svg>
                <p class="title is-3 ml-3">{ "BlockPlot" }</p>
            </Anchor>
        }
    }

    // Construct navend section of navbar
    fn view_navend(&self) -> Html {
        let login_signup_ui = match self.props.session.as_ref() {
            Some(session) => {
                html! {
                    <a href="http://localhost:8000/logout" class="button is-light">
                        { "Log Out" }
                    </a>
                }
            },
            None => {
                html! {
                    <>
                        <a href="http://localhost:8000/auth0" class="button is-primary">
                            <strong>{ "Sign Up" }</strong>
                        </a>
                        <a href="http://localhost:8000/auth0" class="button is-light">
                            { "Log In" }
                        </a>
                    </>
                }
            }
        };
        let picture_div = match self.props.session.as_ref() {
            Some(session) => {
                html! {
                    <figure class="image is-96x96">
                        <img class="is-rounded" src={ session.picture.as_str() }/>
                    </figure>
                }
            },
            None => {
                html! {
                    <>
                    </>
                }
            }
        };

        html! {
            <>
                <div class="navbar-item">
                    <div class="buttons">
                        { login_signup_ui }
                    </div>
                </div>
                <div style="padding-top:0.5rem">
                    { picture_div }
                </div>
            </>
        }
    }

    // Construct main section of navbar
    fn view_navstart(&self) -> Html {
        type Anchor = RouterAnchor<Route>;
        let user_div = match self.props.session.as_ref() {
            Some(session) => {
                html! {
                    <>
                        <Anchor route=Route::UserPage classes="navbar-item">
                            <p class="is-size-4">{ format!("Hi, {}!", &session.given_name) }</p>
                        </Anchor>
                        <Anchor route=Route::FormPage classes="navbar-item">
                            <p class="is-size-4">{ "Create" }</p>
                        </Anchor>
                    </>
                }
            },
            None => {
                html! {
                    <p class="is-size-4 navbar-item">{ "Welcome!" }</p>
                }
            }
        };
        
        html! {
            <>
                { user_div }
                <p class="is-size-4 navbar-item">{ "Explore" }</p>
                <Anchor route=Route::AboutPage classes="navbar-item">
                    <p class="is-size-4">{ "About" }</p>
                </Anchor>
            </>
        }
    }
}

impl Component for NavbarElement {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <Container>
                { self.view_navbar() }
            </Container>
        }
    }
}