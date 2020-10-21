use crate::types::AnalyticData;
use anyhow::Error;
use yew::callback::Callback;
use yew::format::{ Json, Nothing };
use yew::services::fetch::{ FetchService, FetchTask, Request, Response };

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;

pub fn get_timesheets(callback: FetchCallback<AnalyticData>) -> FetchTask {
    let request = Request::get("http://localhost:8000/times")
        .body(Nothing)
        .unwrap();
    
    FetchService::fetch(request, callback).unwrap()
}

pub fn get_dev_timesheets(callback: FetchCallback<AnalyticData>) -> FetchTask {
    let request = Request::get("http://localhost:8000/api/v1/categories/software_developement")
        .body(Nothing)
        .unwrap();

    FetchService::fetch(request, callback).unwrap()
}