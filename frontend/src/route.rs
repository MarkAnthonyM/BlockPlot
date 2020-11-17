use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum Route {
    #[to = "/form"]
    FormPage,
    #[to = "/user"]
    UserPage,
}