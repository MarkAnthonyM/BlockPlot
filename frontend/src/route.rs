use yew_router::prelude::*;

// Enumeration containing all possible routes
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
    #[to = "/unauthorized"]
    UnauthorizedPage,
    #[to = "/user"]
    UserPage,
    #[to = "/"]
    HomePage,
}
