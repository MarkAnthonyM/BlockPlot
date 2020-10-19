use crate::types::AnalyticData;
use anyhow::Error;
use yew::callback::Callback;
use yew::format::{ Json, Nothing };
use yew::services::fetch::{ FetchService, FetchTask, Request, Response };