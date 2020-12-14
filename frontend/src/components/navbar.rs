use crate::route::Route;

use ybc::{ Container, Navbar };

use yew::prelude::*;

use yew_router::components::RouterAnchor;

pub struct NavbarElement {}

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
        html! {
            <div class="navbar-end">
                <div class="navbar-item">
                    <div class="buttons">
                        <a href="http://localhost:8000/auth0" class="button is-primary">
                            <strong>{ "Sign Up" }</strong>
                        </a>
                        <a href="http://localhost:8000/auth0" class="button is-light">
                            { "Log In" }
                        </a>
                        <a href="http://localhost:8000/logout" class="button is-light">
                            { "Log Out" }
                        </a>
                    </div>
                </div>
            </div>
        }
    }

    // Construct main section of navbar
    fn view_navstart(&self) -> Html {
        type Anchor = RouterAnchor<Route>;
        
        html! {
            <>
                <Anchor route=Route::UserPage classes="navbar-item">
                    <p class="is-size-4">{ "UserName" }</p>
                </Anchor>
                <Anchor route=Route::FormPage classes="navbar-item">
                    <p class="is-size-4">{ "Create" }</p>
                </Anchor>
                <Anchor route=Route::AboutPage classes="navbar-item">
                    <p class="is-size-4">{ "About" }</p>
                </Anchor>
            </>
        }
    }
}

impl Component for NavbarElement {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Container>
                { self.view_navbar() }
            </Container>
        }
    }
}