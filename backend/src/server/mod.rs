use std::{
    net::{SocketAddr, ToSocketAddrs},
    slice::from_ref,
    sync::Mutex,
};

use actix_cors::Cors;
use actix_web::{
    http, middleware,
    web::{get, Data},
    App, HttpServer,
};
use anyhow::{format_err, Result};

use crate::api;
use crate::Broadcaster;

use url::Url;
mod test;

pub struct Server {
    config: (),
    runner: actix::SystemRunner,
    url: url::Url,
}

impl Server {
    pub fn create_default(broadcaster_data: Data<Mutex<Broadcaster>>) -> Result<Self> {
        let runner = actix::System::new("stellarust_backend");

        let url = Url::parse("http://127.0.0.1:8000")?;

        let mut server = HttpServer::new(move || {
            let cors = get_default_cors();
            App::new()
                .wrap(cors)
                .app_data(broadcaster_data.clone())
                .wrap(middleware::Logger::default())
                .service(api::echo_json_file)
                .service(api::get_json_file)
                .service(api::new_client)
                .service(api::broadcast)
        });

        let addrs = Self::url_to_socket_addrs(&url)?;

        server.bind(addrs.as_slice())?.run();

        Ok(Self {
            config: (),
            runner: runner,
            url: url,
        })
    }

    fn url_to_socket_addrs(url: &Url) -> Result<Vec<SocketAddr>> {
        let host = url
            .host()
            .ok_or_else(|| format_err!("No host name in the URL"))?;
        let port = url
            .port_or_known_default()
            .ok_or_else(|| format_err!("No port in the URL"))?;
        let addrs;
        let addr;
        Ok(match host {
            url::Host::Domain(domain) => {
                addrs = (domain, port).to_socket_addrs()?;
                addrs.as_slice().to_owned()
            }
            url::Host::Ipv4(ip) => {
                addr = (ip, port).into();
                from_ref(&addr).to_owned()
            }
            url::Host::Ipv6(ip) => {
                addr = (ip, port).into();
                from_ref(&addr).to_owned()
            }
        })
    }
}

fn get_default_cors() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:3000")
        .allowed_origin("http://127.0.0.1:3000")
        .allowed_methods(vec![http::Method::GET])
        .allowed_headers(vec![
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
            http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
        ])
        .max_age(3600)
}
