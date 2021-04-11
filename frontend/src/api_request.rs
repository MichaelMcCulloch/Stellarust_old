use serde::{Deserialize, Serialize};
use std::fmt::{Error, Formatter};
use std::future::Future;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use yew::{
    format::Json,
    prelude::*,
    web_sys::{Request, RequestInit, RequestMode, Response, Window},
};

pub fn send_future<COMP: Component, F>(link: ComponentLink<COMP>, future: F)
where
    F: Future<Output = COMP::Message> + 'static,
{
    spawn_local(async move {
        link.send_message(future.await);
    });
}
#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}
impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        std::fmt::Debug::fmt(&self.err, f)
    }
}
impl std::error::Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        FetchError { err: value }
    }
}

/// The possible states a fetch request can be in.
pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(FetchError),
}
async fn fetch_json(url: &'static str) -> Result<Json<MyJsonFile>, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::NoCors);

    let request = Request::new_with_str_and_init(url, &opts)?;

    let window: Window = yew::web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    assert!(resp_value.is_instance_of::<Response>());

    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    Ok(Json(MyJsonFile {
        name: "what is up".into(),
        number: 7,
    }))
}

#[derive(Debug, Serialize, Deserialize)]
struct MyJsonFile {
    name: String,
    number: i32,
}
const MARKDOWN_URL: &str = "http://localhost:8000/json_get/";

pub enum Msg {
    SetFetchState(FetchState<Json<MyJsonFile>>),
    GetJson,
}
pub struct ApiComponent {
    json: FetchState<Json<MyJsonFile>>,
    link: ComponentLink<Self>,
}

impl Component for ApiComponent {
    type Message = Msg;

    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ApiComponent {
            json: FetchState::NotFetching,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetFetchState(fetch_state) => {
                self.json = fetch_state;
                true
            }
            Msg::GetJson => {
                let future = async {
                    match fetch_json(MARKDOWN_URL).await {
                        Ok(md) => Msg::SetFetchState(FetchState::Success(md)),
                        Err(err) => Msg::SetFetchState(FetchState::Failed(err)),
                    }
                };
                send_future(self.link.clone(), future);
                self.link
                    .send_message(Msg::SetFetchState(FetchState::Fetching));
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match &self.json {
            FetchState::NotFetching => html! {
                <>
                    <button onclick=self.link.callback(|_| Msg::GetJson)>
                        {"Get Markdown"}
                    </button>
                </>
            },
            FetchState::Fetching => html! {"Fetching"},
            FetchState::Success(data) => html! {
                <>
                    <p> {&data.0.name} </p>
                    <p> {&data.0.number} </p>
                </>
            },
            FetchState::Failed(err) => html! {&err},
        }
    }
}
