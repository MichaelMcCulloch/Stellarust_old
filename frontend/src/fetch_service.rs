use anyhow::Error;
use serde_derive::Deserialize;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew::{
    format::{Json, Nothing},
    services::{
        fetch::{Request, Response},
        FetchService,
    },
};
extern crate common;
use common::MyJsonFile;

pub enum Msg {
    FetchData,
    FetchReady(Result<MyJsonFile, Error>),
    Ignore,
}

pub struct FetchComponent {
    link: ComponentLink<FetchComponent>,
    fetching: bool,
    data: Option<MyJsonFile>,
    ft: Option<FetchTask>,
}

impl FetchComponent {
    fn view_data(&self) -> Html {
        if let Some(value) = &self.data {
            html! {
                <>
                    <p> { &value.name } </p>
                    <p> { &value.number } </p>
                </>
            }
        } else {
            html! {
                <p>{ "Data hasn't fetched yet." }</p>
            }
        }
    }
    fn fetch_json(&mut self) -> yew::services::fetch::FetchTask {
        let callback =
            self.link
                .callback(move |response: Response<Json<Result<MyJsonFile, Error>>>| {
                    let (meta, Json(data)) = response.into_parts();
                    println!("META: {:?}, {:?}", meta, data);
                    if meta.status.is_success() {
                        Msg::FetchReady(data)
                    } else {
                        Msg::Ignore // FIXME: Handle this error accordingly.
                    }
                });
        let request = Request::get("http://127.0.0.1:8000/json_get")
            .header("Access-Control-Allow-Origin", "http://127.0.0.1:8000")
            .body(Nothing)
            .expect("Could not form request");
        FetchService::fetch(request, callback).unwrap()
    }
}

impl Component for FetchComponent {
    type Message = Msg;

    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        FetchComponent {
            link,
            fetching: false,
            data: None,

            ft: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::FetchData => {
                self.fetching = true;
                let task = self.fetch_json();
                self.ft = Some(task);
                true
            }
            Msg::FetchReady(response) => {
                self.fetching = false;
                self.data = response.ok();
                true
            }
            Msg::Ignore => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        todo!()
    }

    fn view(&self) -> yew::Html {
        html! {
            <div>
                <nav class="menu">
                    <button onclick=self.link.callback(|_| Msg::FetchData)>
                        { "Fetch Data" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::FetchData)>
                        { "Fetch Data [binary]" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::FetchData)>
                        { "Fetch Data [toml]" }
                    </button>
                    { self.view_data() }

                </nav>
            </div>
        }
    }
}
