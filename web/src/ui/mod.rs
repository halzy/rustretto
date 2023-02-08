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
use hyper::{header::HeaderName, Request, StatusCode};
use tower::util::ServiceExt;
use tower_http::{
    request_id::{MakeRequestId, RequestId, SetRequestIdLayer},
    services::ServeDir,
};

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

async fn root(live: LiveViewUpgrade) -> impl IntoResponse {
    live.response(|embed_live_view| {
        let history = History::new(&embed_live_view);
        let prompt = Prompt::new(&embed_live_view);

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