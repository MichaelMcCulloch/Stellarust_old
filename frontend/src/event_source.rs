use std::fmt;

use gloo::events::EventListener;
use std::borrow::Cow;
use wasm_bindgen::JsCast;
use web_sys::{Event, EventSource, MessageEvent};
use yew::callback::Callback;
use yew::format::{FormatError, Text};
use yew::services::Task;

#[derive(Debug, PartialEq)]
pub enum EventSourceStatus {
    Open,
    Error,
}

#[derive(Debug, PartialEq)]
pub enum ReadyState {
    Connecting,
    Open,
    Closed,
}

pub struct EventSourceTask {
    event_source: EventSource,
    _notificiation: Callback<EventSourceStatus>,
    listeners: Vec<EventListener>,
}

impl EventSourceTask {
    #![allow(clippy::clippy::unnecessary_wraps)]
    fn new(
        event_source: EventSource,
        notification: Callback<EventSourceStatus>,
    ) -> Result<EventSourceTask, &'static str> {
        Ok(EventSourceTask {
            event_source,
            _notificiation: notification,
            listeners: vec![],
        })
    }

    fn add_unwrapped_event_listener<S, F>(&mut self, event_type: S, callback: F)
    where
        S: Into<Cow<'static, str>>,
        F: FnMut(&Event) + 'static,
    {
        self.listeners
            .push(EventListener::new(&self.event_source, event_type, callback));
    }

    pub fn add_event_listener<S, OUT: 'static>(&mut self, event_type: S, callback: Callback<OUT>)
    where
        S: Into<Cow<'static, str>>,
        OUT: From<Text>,
    {
        let wrapped_callback = move |event: &Event| {
            let event = event.dyn_ref::<MessageEvent>().unwrap();
            let text = event.data().as_string();

            let data = if let Some(text) = text {
                Ok(text)
            } else {
                Err(FormatError::ReceivedBinaryForText.into())
            };

            let out = OUT::from(data);
            callback.emit(out);
        };
        self.add_unwrapped_event_listener(event_type, wrapped_callback)
    }

    pub fn ready_state(&self) -> ReadyState {
        match self.event_source.ready_state() {
            web_sys::EventSource::CONNECTING => ReadyState::Connecting,
            web_sys::EventSource::OPEN => ReadyState::Open,
            web_sys::EventSource::CLOSED => ReadyState::Closed,
            _ => panic!("unexpected ready state"),
        }
    }
}

impl fmt::Debug for EventSourceTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("EventSourceTask")
    }
}

#[derive(Default, Debug)]
pub struct EventSourceService {}

impl EventSourceService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn connect(
        &mut self,
        url: &str,
        noticiation: Callback<EventSourceStatus>,
    ) -> Result<EventSourceTask, &str> {
        let event_source = EventSource::new(url);
        if event_source.is_err() {
            return Err("Faild to create event source with given url");
        }
        let event_source = event_source.map_err(|_| "failed to build event source")?;

        let notify = noticiation.clone();
        let listener_open = move |_: &Event| {
            notify.emit(EventSourceStatus::Open);
        };
        let notify = noticiation.clone();
        let listener_error = move |_: &Event| {
            notify.emit(EventSourceStatus::Error);
        };

        let mut result = EventSourceTask::new(event_source, noticiation)?;
        result.add_unwrapped_event_listener("open", listener_open);
        result.add_unwrapped_event_listener("error", listener_error);
        Ok(result)
    }
}

impl Task for EventSourceTask {
    fn is_active(&self) -> bool {
        self.ready_state() == ReadyState::Open
    }
}

impl Drop for EventSourceTask {
    fn drop(&mut self) {
        if self.is_active() {
            self.event_source.close();
        }
    }
}
