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
                <img src="../static/logo_rough.svg" />
            </Anchor>
        }
    }

    // Construct navend section of navbar
    fn view_navend(&self) -> Html {
        html! {

        }
    }

    // Construct main section of navbar
    fn view_navstart(&self) -> Html {
        type Anchor = RouterAnchor<Route>;
        
        html! {
            <>
                <Anchor route=Route::UserPage classes="navbar-item">
                    { "UserName" }
                </Anchor>
                <Anchor route=Route::FormPage classes="navbar-item">
                    { "Create" }
                </Anchor>
                <Anchor route=Route::AboutPage classes="navbar-item">
                    { "About" }
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