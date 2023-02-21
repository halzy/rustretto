mod history;
mod prompt;

use std::io;

use axum::{
    handler::HandlerWithoutStateExt,
    http::HeaderValue,
    response::IntoResponse,
    routing::{get, get_service, MethodRouter},
};
use axum_live_view::{html, LiveViewUpgrade};
use bastion::prelude::*;
use breach::ViewId;
use hyper::{header::HeaderName, HeaderMap, Request, StatusCode};
use tower::util::ServiceExt;
use tower_http::{
    request_id::{MakeRequestId, RequestId, SetRequestIdLayer},
    services::ServeDir,
};

use crate::view_registration_guard::MessageListener;

use self::{history::History, prompt::Prompt};
fn asset_router() -> MethodRouter {
    async fn handle_404() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "Not found")
    }

    // you can convert handler function to service
    let service = handle_404
        .into_service()
        .map_err(|err| -> std::io::Error { match err {} });

    let serve_dir = ServeDir::new("assets").not_found_service(service);
    let serve_dir = get_service(serve_dir).handle_error(handle_error);

    serve_dir
}

#[derive(Clone, Default)]
struct UUIDMakeRequestId {}

impl MakeRequestId for UUIDMakeRequestId {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        // generate UUID
        let request_id: HeaderValue = uuid::Uuid::new_v4()
            .to_string()
            .parse()
            .expect("can always parse a uuid");
        Some(RequestId::new(request_id))
    }
}
fn add_request_id() -> SetRequestIdLayer<UUIDMakeRequestId> {
    let x_request_id = HeaderName::from_static("x-request-id");
    SetRequestIdLayer::new(x_request_id.clone(), UUIDMakeRequestId::default())
}

pub async fn start(listen_port: u16) -> Result<(), ()> {
    let serve_dir = asset_router();
    // Set up Axum here
    // build our application with a single route
    let app = axum::Router::new()
        .route("/", get(root))
        .route_layer(add_request_id())
        // Use a precompiled and minified build of axum-live-view's JavaScript.
        // This is the easiest way to get started. Integration with bundlers
        // is of course also possible.
        .route("/assets/live-view.js", axum_live_view::precompiled_js())
        .fallback_service(serve_dir);

    // run it with hyper on localhost:3000
    axum::Server::bind(&([0, 0, 0, 0], listen_port).into())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

async fn root(live: LiveViewUpgrade, headers: HeaderMap) -> impl IntoResponse {
    let view_id = headers
        .get("x-request-id")
        .expect("All liveview requests have x-request-id header")
        .to_str()
        .map(|id| ViewId::new(id))
        .expect("x-request-id can become a str");

    live.response(move |embed_live_view| {
        // create supervisor to manage this component
        tracing::error!(
            %view_id,
            is_live = embed_live_view.connected(),
            "will it blend"
        );
        let component_supervisor = create_component_supervisor(embed_live_view.connected())
            .expect("Can create component supervisor");

        // create the guard, it is responsible for registering the view with the breach
        let message_listener = component_supervisor.map(|cs| MessageListener::new(view_id, cs));

        // UI components
        let history = History::new(message_listener.clone());
        let prompt = Prompt::new(message_listener.clone());

        let combined_view =
            axum_live_view::live_view::combine((history, prompt), |history, prompt| {
                html! {
                    <div>{ history }</div>
                    <div>{ prompt }</div>
                }
            });

        html! {
            <!DOCTYPE html>
            <html>
                <head>
                    <link href="/app.css" rel="stylesheet" />
                </head>
                <body>
                    // Embed our live view into the HTML template. This will render the
                    // view and include the HTML in the response, leading to good SEO
                    // and fast first paint.
                    { embed_live_view.embed(combined_view) }

                    // Load the JavaScript. This will automatically initialize live view
                    // connections.
                    <script src="/assets/live-view.js"></script>
                </body>
            </html>
        }
    })
}

fn create_component_supervisor(is_live: bool) -> Result<Option<SupervisorRef>, ()> {
    // If we are in a live_view and not the first time render we create a Supervisor
    is_live
        .then(|| Bastion::supervisor(|sp| sp.with_strategy(SupervisionStrategy::OneForOne)))
        .transpose()
}
