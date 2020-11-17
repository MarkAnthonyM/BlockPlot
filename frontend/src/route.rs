use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum Route {
    #[to = "/about"]
    AboutPage,
    #[to = "/form"]
    FormPage,
    #[to = "/user"]
    UserPage,
    #[to = "/"]
    HomePage,
}