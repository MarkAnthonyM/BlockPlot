use anyhow::Error;
use crate::types::{ Session, TimeWrapper};
use yew::callback::Callback;
use yew::format::{ Json, Nothing };
use yew::services::fetch::{ FetchOptions, FetchService, FetchTask, Request, Response };
use yew::web_sys::RequestCredentials;

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;

// A testing request getter. Will probably remove at conclusion of tests
pub fn get_dev_skillblocks(callback: FetchCallback<TimeWrapper>) -> FetchTask {
    let url = format!("http://localhost:8000/api/skillblocks");
    let request = Request::get(url)
        .body(Nothing)
        .unwrap();
    let options = FetchOptions {
        credentials: Some(RequestCredentials::Include),
        ..FetchOptions::default()
    };

    FetchService::fetch_binary_with_options(request, options, callback).unwrap()
}

// Fetch user session information
pub fn get_user_session(callback: FetchCallback<Session>) -> FetchTask {
    let url = format!("http://localhost:8000/home");
    let request = Request::get(url)
        .body(Nothing)
        .unwrap();
    let options = FetchOptions {
        credentials: Some(RequestCredentials::Include),
        ..FetchOptions::default()
    };

    FetchService::fetch_binary_with_options(request, options, callback).unwrap()
}