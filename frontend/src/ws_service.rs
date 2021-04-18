use anyhow::Error;
use serde_derive::{Deserialize, Serialize};
use yew::format::Json;
use yew::prelude::*;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

#[derive(Debug, Deserialize, Serialize)]
pub struct MyJsonFile {
    name: String,
    number: i32,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum SubscriptionAction {
    Subscribe,
    Unsubscribe,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SubscriptionRequest {
    action: SubscriptionAction,
    empire_name: String,
}

pub enum WsAction {
    Connect,
    SendData,
    Disconnect,
    Lost,
}

pub enum Msg {
    WsAction(WsAction),
    WsReady(Result<MyJsonFile, Error>),
    Ignore,
}

impl From<WsAction> for Msg {
    fn from(action: WsAction) -> Self {
        Msg::WsAction(action)
    }
}
pub struct WebsocketComponent {
    link: ComponentLink<WebsocketComponent>,
    data: Option<MyJsonFile>,
    ws: Option<WebSocketTask>,
}

impl Component for WebsocketComponent {
    type Message = Msg;

    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        WebsocketComponent {
            link,
            data: None,
            ws: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::WsAction(action) => match action {
                WsAction::Connect => {
                    let callback = self.link.callback(|Json(data)| Msg::WsReady(data));
                    let notification = self.link.callback(|status| match status {
                        WebSocketStatus::Opened => Msg::Ignore,
                        WebSocketStatus::Closed | WebSocketStatus::Error => WsAction::Lost.into(),
                    });
                    let task =
                        WebSocketService::connect("ws://127.0.0.1:8000/", callback, notification)
                            .unwrap();
                    self.ws = Some(task);
                    true
                }
                WsAction::SendData => {
                    let request = MyJsonFile {
                        name: "websocket out".into(),
                        number: 23,
                    };
                    self.ws.as_mut().unwrap().send(Json(&request));
                    true
                }
                WsAction::Disconnect => {
                    self.ws.take();
                    true
                }
                WsAction::Lost => {
                    self.ws = None;
                    true
                }
            },
            Msg::WsReady(response) => {
                self.data = response.ok();
                true
            }
            Msg::Ignore => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <nav class="menu">
                    { self.view_data() }
                    <button disabled=self.ws.is_some()
                            onclick=self.link.callback(|_| WsAction::Connect)>
                        { "Connect To WebSocket" }
                    </button>
                    <button disabled=self.ws.is_none()
                            onclick=self.link.callback(|_| WsAction::SendData)>
                        { "Send To WebSocket" }
                    </button>
                    <button disabled=self.ws.is_none()
                            onclick=self.link.callback(|_| WsAction::Disconnect)>
                        { "Close WebSocket connection" }
                    </button>
                </nav>
            </div>
        }
    }
}

impl WebsocketComponent {
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
}
