use crate::assists::error::beautify_error;
use crate::assists::notify::{notify, NotifyType};
use enki_shared::proxy::ProxyData;
use leptos::*;
use tauri_sys::tauri;

pub const PROXY_PROTOCOLS: &[&str] = &["direct", "http", "https", "socks5"];

pub struct ProxyCtx {
    pub hidden: ReadSignal<bool>,
    pub set_hidden: WriteSignal<bool>,
    pub protocol: ReadSignal<String>,
    pub set_protocol: WriteSignal<String>,
    pub server: ReadSignal<String>,
    pub set_server: WriteSignal<String>,
    pub port: ReadSignal<String>,
    pub set_port: WriteSignal<String>,
    pub load_proxy: Resource<(), ()>,
    pub save_proxy: Action<(), ()>,
}

pub fn use_proxy_ctx(cx: Scope) -> ProxyCtx {
    let (hidden, set_hidden) = create_signal(cx, true);
    let (protocol, set_protocol) = create_signal(cx, String::new());
    let (server, set_server) = create_signal(cx, String::new());
    let (port, set_port) = create_signal(cx, String::new());

    let load_proxy = create_resource(
        cx,
        move || (),
        move |_| async move {
            match tauri::invoke::<_, ProxyData>("plugin:proxy|load_proxy_data", &()).await {
                Err(err) => {
                    error!("load proxy data failed: {:?}", err);
                    notify(cx, NotifyType::Error, beautify_error(err));
                }
                Ok(proxy) => {
                    set_protocol(proxy.protocol);
                    set_server(proxy.server);
                    set_port(proxy.port);
                }
            }
        },
    );

    let save_proxy = create_action(cx, move |_| async move {
        let proxy = ProxyData {
            protocol: protocol.get(),
            server: server.get(),
            port: port.get(),
        };

        if let Err(err) = tauri::invoke::<_, ()>("plugin:proxy|save_proxy_data", &proxy).await {
            error!("{:?}", err);
            notify(cx, NotifyType::Error, beautify_error(err));
            return;
        }

        set_hidden(true);
    });

    ProxyCtx {
        hidden,
        set_hidden,
        protocol,
        set_protocol,
        server,
        set_server,
        port,
        set_port,
        load_proxy,
        save_proxy,
    }
}
