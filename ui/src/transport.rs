use leptos::*;
use leptos_use::utils::JsonCodec;

use crate::{collection::MediaEvent, log};

pub trait Transport: 'static + Copy {
    type Error: Clone + std::fmt::Debug;
    fn subscribe(&self) -> Signal<Option<MediaEvent>>;
    async fn send(&self, ev: MediaEvent) -> Result<MediaEvent, Self::Error>;
}

#[derive(Clone, Copy)]
pub struct ReqSSETransport {}

impl ReqSSETransport {
    pub fn new() -> Self {
        Self {}
    }
}

impl Transport for ReqSSETransport {
    type Error = String;
    fn subscribe(&self) -> Signal<Option<MediaEvent>> {
        let event_source = leptos_use::use_event_source::<MediaEvent, JsonCodec>(
            "http://localhost:8080/api/events",
        );
        create_effect(move |_| {
            log!("{:?}", event_source.data.get());
            log!(
                "{:?}",
                event_source
                    .error
                    .with(|e| e.as_ref().map(|e| e.to_string()))
            );
        });
        return event_source.data.into();
    }

    async fn send(&self, ev: MediaEvent) -> Result<MediaEvent, Self::Error> {
        log!("{:?}", ev);
        match ev {
            MediaEvent::Update {
                ref id,
                ref field,
                ref value,
            } => {
                match gloo_net::http::Request::patch(&format!(
                    "http://localhost:8080/api/media/{}",
                    id
                ))
                .query([("f", &field), ("v", &value)])
                .send()
                .await
                {
                    Ok(res) => match res.status() {
                        204 => Ok(ev),
                        _ => Err("bad status".to_owned()),
                    },
                    Err(e) => Err(e.to_string()),
                }
            }
            _ => Err("attempt to send unsupported event to remote".to_owned()),
        }
    }
}
