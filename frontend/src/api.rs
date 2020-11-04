// use crate::types::TimeData;
use crate::types::TimeWrapper;
use anyhow::Error;
use chrono::prelude::*;
use yew::services::console::ConsoleService;
use yew::callback::Callback;
use yew::format::{ Json, Nothing };
use yew::services::fetch::{ FetchService, FetchTask, Request, Response };

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;

// pub fn get_dev_skillblocks(callback: FetchCallback<TimeData>) -> FetchTask {
//     let category = "software%20development";
//     let end_date = Local::now().date().naive_utc();
//     let begin_date = NaiveDate::from_ymd(end_date.year() - 1, end_date.month(), end_date.day() + 1);
//     let formatted_url = format!("http://localhost:8000/api/categories/{}?begin_date={}&end_date={}", category, begin_date, end_date);
//     ConsoleService::log(&formatted_url);
//     let request = Request::get(formatted_url)
//         .body(Nothing)
//         .unwrap();

//     FetchService::fetch(request, callback).unwrap()
// }

pub fn get_dev_skillblocks(callback: FetchCallback<TimeWrapper>) -> FetchTask {
    let url = format!("http://localhost:8000/api/categories/multi");
    let request = Request::get(url)
        .body(Nothing)
        .unwrap();

    FetchService::fetch(request, callback).unwrap()
}