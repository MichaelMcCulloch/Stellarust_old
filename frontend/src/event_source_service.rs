use std::fmt::format;

use anyhow::Error;
use yew::{prelude::*, services::ConsoleService};

use yew_event_source::{EventSourceService, EventSourceStatus, EventSourceTask};

pub enum EventSourceAction {
    Connect,
    Disconnect,
    Lost,
}

pub enum Msg {
    EsReady(Result<String, Error>),
    EsCheckState,
}
pub struct EventSourceComponent {
    link: ComponentLink<EventSourceComponent>,
    data: Option<String>,
    es: EventSourceTask,
}

impl Component for EventSourceComponent {
    type Message = Msg;

    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let task = {
            let callback = link.callback(|data| Msg::EsReady(data));
            let notification = link.callback(|status| {
                if status == EventSourceStatus::Error {
                    log::error!("event source error");
                }
                Msg::EsCheckState
            });
            let mut task = EventSourceService::new()
                .connect("/events", notification)
                .unwrap();
            task.add_event_listener("message", callback);
            task
        };

        EventSourceComponent {
            link,
            data: None,
            es: task,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::EsReady(response) => {
                match response {
                    Ok(data_result) => {
                        self.data = Some(data_result);
                    }
                    Err(e) => {
                        log::error!("{}", e);
                    }
                };
                true
            }
            Msg::EsCheckState => true,
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
                </nav>
            </div>
        }
    }
}

impl EventSourceComponent {
    fn view_data(&self) -> Html {
        if let Some(value) = &self.data {
            html! {
                <>
                    <p> { format!("Event source data {:?}", value) } </p>
                </>
            }
        } else {
            html! {
                <>
                    <p>{ "Data hasn't fetched yet." }</p>
                </>
            }
        }
    }
}
