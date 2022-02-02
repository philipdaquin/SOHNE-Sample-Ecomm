use crate::types::Product;
use anyhow::Error;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallBack<T> = Callback<FetchResponse<T>>;

pub fn get_products(callback: FetchCallBack<Vec<Product>>) -> FetchTask { 
    let req = Request::get("/assets/img/products.json")
        .body(Nothing)
        .unwrap();
    FetchService::fetch(req, callback)
        .expect("Could Not Fetch Items")
}

pub fn get_product(id: i32, callback: FetchCallBack<Product>) -> FetchTask { 
    let req = Request::get(format!("/assets/img/{}.json", id))
    .body(Nothing)
    .unwrap();
    FetchService::fetch(req, callback)
    .expect("Could Not Fetch Items")
}