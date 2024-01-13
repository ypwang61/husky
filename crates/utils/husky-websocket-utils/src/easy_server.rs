use serde_impl::IsSerdeImpl;
use std::{
    net::{SocketAddr, ToSocketAddrs},
    sync::Arc,
};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use tokio::sync::Mutex;

pub trait IsEasyWebsocketServer: Send + 'static
where
    <Self::SerdeImpl as IsSerdeImpl>::Error: Send,
{
    /// goes from server to client
    type Response: serde::Serialize + Send + 'static;
    /// goes from client to server
    type Request: for<'a> serde::Deserialize<'a> + Send + 'static;

    type SerdeImpl: IsSerdeImpl;

    fn handle(&mut self, request: Self::Request) -> Option<Self::Response>;

    fn easy_serve(self, addr: impl ToSocketAddrs)
    where
        Self: Sized,
    {
        easy_serve(Arc::new(Mutex::new(self)), addr)
    }
}

#[tokio::main]
pub async fn easy_serve<S>(server: std::sync::Arc<Mutex<S>>, addr: impl ToSocketAddrs)
where
    S: IsEasyWebsocketServer,
    <S::SerdeImpl as IsSerdeImpl>::Error: Send,
{
    match addr.to_socket_addrs() {
        Ok(mut socket_addrs) => {
            let Some(addr) = socket_addrs.next() else {
                todo!()
            };
            easy_server_aux(server, addr).await
        }
        Err(_) => todo!(),
    }
}

async fn easy_server_aux<S>(slf: std::sync::Arc<Mutex<S>>, addr: impl Into<SocketAddr>)
where
    S: IsEasyWebsocketServer,
    <S::SerdeImpl as IsSerdeImpl>::Error: Send,
{
    use tower_http::trace::{DefaultMakeSpan, TraceLayer};
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_websockets=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let router = Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(slf)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );
    let addr = addr.into();
    tracing::debug!("listening on {}", addr);
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn websocket_handler<S>(
    ws: WebSocketUpgrade,
    State(state): State<Arc<Mutex<S>>>,
) -> impl IntoResponse
where
    S: IsEasyWebsocketServer,
    <S::SerdeImpl as IsSerdeImpl>::Error: Send,
{
    println!("connected.");
    ws.on_upgrade(|socket| websocket(socket, state))
}

async fn websocket<S>(mut stream: WebSocket, state: Arc<Mutex<S>>)
where
    S: IsEasyWebsocketServer,
    <S::SerdeImpl as IsSerdeImpl>::Error: Send,
{
    println!("Websocket connection established.");
    while let Some(request) = stream.recv().await {
        match request {
            Ok(request) => match request {
                Message::Text(request) => match S::SerdeImpl::from_str(&request) {
                    Ok(request) => {
                        let Some(response) = state.lock().await.handle(request) else {
                            continue;
                        };
                        match S::SerdeImpl::to_string(&response) {
                            Ok(response) => {
                                if let Err(_e) = stream.send(Message::Text(response)).await {
                                    todo!()
                                }
                            }
                            Err(e) => todo!("e = {e}"),
                        }
                    }
                    Err(_) => todo!(),
                },
                Message::Binary(_) => todo!(),
                Message::Ping(_) => todo!(),
                Message::Pong(_) => todo!(),
                Message::Close(_) => todo!(),
            },
            Err(e) => eprintln!("e = {e}"),
        }
    }
    todo!()
}
