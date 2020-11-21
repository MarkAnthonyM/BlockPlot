use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum Route {
    #[to = "/about"]
    AboutPage,
    #[to = "/form"]
    FormPage,
    #[to = "/sign_in"]
    SignInPage,
    #[to = "/sign_up"]
    SignUpPage,
    #[to = "/user"]
    UserPage,
    #[to = "/"]
    HomePage,
}