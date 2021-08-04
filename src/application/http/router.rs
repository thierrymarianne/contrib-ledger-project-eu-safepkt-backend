use crate::app::controller::source::save_source;
use crate::app::controller::verification_step;
use crate::app::middleware;
use hyper::{Body, Response, StatusCode};
use routerify::{Middleware, RequestInfo, Result, Router, RouterService};
use std::convert::Infallible;
use tracing::error;

async fn error_handler(err: routerify::RouteError, _: RequestInfo) -> Response<Body> {
    error!("Routing error: {}", err);
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(format!("Sorry, something went wrong.")))
        .unwrap()
}

pub fn new_router() -> Result<RouterService<Body, Infallible>> {
    let router = Router::builder()
        .middleware(Middleware::pre(middleware::logger::log_handler))
        .post("/source", save_source)
        .get("/steps", verification_step::get_steps)
        .post(
            "/:stepName/:projectId",
            verification_step::start_running_step,
        )
        .get(
            "/:stepName/:projectId/report",
            verification_step::get_step_report,
        )
        .get(
            "/:stepName/:projectId/progress",
            verification_step::get_step_progress,
        )
        .err_handler_with_info(error_handler)
        .build()
        .unwrap();

    RouterService::new(router)
}
