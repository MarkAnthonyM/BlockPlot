use crate::types::TimeWrapper;
use anyhow::Error;
use yew::callback::Callback;
use yew::format::{ Json, Nothing };
use yew::services::fetch::{ FetchService, FetchTask, Request, Response };

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;

// A testing request getter. Will probably remove at conclusion of tests
pub fn get_dev_skillblocks(callback: FetchCallback<TimeWrapper>) -> FetchTask {
    let url = format!("http://localhost:8000/api/skillblocks");
    let request = Request::get(url)
        .body(Nothing)
        .unwrap();

    FetchService::fetch(request, callback).unwrap()
}