//! Web framework integrations for v0.5.0
//!
//! First-class integration with popular Rust web frameworks

#[cfg(feature = "async")]
use crate::{TemplateEngine, TemplateContext, async_engine::AsyncTemplateEngine};

// Axum integration
#[cfg(feature = "axum-integration")]
pub mod axum_integration {
    use super::*;
    use axum::response::{Html, Response, IntoResponse};
    use axum::http::StatusCode;
    use futures::future::BoxFuture;

    /// Axum response extension for TemplateEngine
    pub trait AxumTemplateEngine {
        /// Render template and return Axum HTML response
        fn render_axum<'a>(&'a mut self, template: &'a str, context: &'a TemplateContext) -> BoxFuture<'a, Result<Html<String>, TemplateResponseError>>;
        
        /// Render template file and return Axum HTML response
        fn render_axum_file<'a>(&'a mut self, template_name: &'a str, context: &'a TemplateContext) -> BoxFuture<'a, Result<Html<String>, TemplateResponseError>>;
    }

    #[derive(Debug)]
    pub struct TemplateResponseError {
        pub message: String,
        pub status: StatusCode,
    }

    impl IntoResponse for TemplateResponseError {
        fn into_response(self) -> Response {
            (self.status, self.message).into_response()
        }
    }

    impl From<crate::TemplateError> for TemplateResponseError {
        fn from(err: crate::TemplateError) -> Self {
            TemplateResponseError {
                message: err.to_string(),
                status: StatusCode::INTERNAL_SERVER_ERROR,
            }
        }
    }

    impl AxumTemplateEngine for TemplateEngine {
        fn render_axum<'a>(&'a mut self, template: &'a str, context: &'a TemplateContext) -> BoxFuture<'a, Result<Html<String>, TemplateResponseError>> {
            Box::pin(async move {
                match self.render_string_async(template, context).await {
                    Ok(rendered) => Ok(Html(rendered)),
                    Err(e) => Err(e.into()),
                }
            })
        }

        fn render_axum_file<'a>(&'a mut self, template_name: &'a str, context: &'a TemplateContext) -> BoxFuture<'a, Result<Html<String>, TemplateResponseError>> {
            Box::pin(async move {
                match self.render_async(template_name, context).await {
                    Ok(rendered) => Ok(Html(rendered)),
                    Err(e) => Err(e.into()),
                }
            })
        }
    }
}

// Warp integration
#[cfg(feature = "warp-integration")]
pub mod warp_integration {
    use super::*;
    use warp::reply::{Reply, Response};
    // Note: StatusCode available if needed for future error handling
    use futures::future::BoxFuture;

    /// Warp response extension for TemplateEngine
    pub trait WarpTemplateEngine {
        /// Render template and return Warp reply
        fn render_warp<'a>(&'a mut self, template: &'a str, context: &'a TemplateContext) -> BoxFuture<'a, Result<impl Reply, crate::TemplateError>>;
    }

    pub struct TemplateReply {
        content: String,
    }

    impl Reply for TemplateReply {
        fn into_response(self) -> Response {
            warp::reply::html(self.content).into_response()
        }
    }

    impl WarpTemplateEngine for TemplateEngine {
        fn render_warp<'a>(&'a mut self, template: &'a str, context: &'a TemplateContext) -> BoxFuture<'a, Result<impl Reply, crate::TemplateError>> {
            Box::pin(async move {
                match self.render_string_async(template, context).await {
                    Ok(rendered) => Ok(TemplateReply { content: rendered }),
                    Err(e) => Err(e),
                }
            })
        }
    }
}

// Actix-web integration
#[cfg(feature = "actix-integration")]
pub mod actix_integration {
    use super::*;
    use actix_web::{HttpResponse, Result as ActixResult};
    use futures::future::BoxFuture;

    /// Actix-web response extension for TemplateEngine
    pub trait ActixTemplateEngine {
        /// Render template and return Actix HttpResponse
        fn render_actix<'a>(&'a mut self, template: &'a str, context: &'a TemplateContext) -> BoxFuture<'a, ActixResult<HttpResponse>>;
    }

    impl ActixTemplateEngine for TemplateEngine {
        fn render_actix<'a>(&'a mut self, template: &'a str, context: &'a TemplateContext) -> BoxFuture<'a, ActixResult<HttpResponse>> {
            Box::pin(async move {
                match self.render_string_async(template, context).await {
                    Ok(rendered) => Ok(HttpResponse::Ok()
                        .content_type("text/html; charset=utf-8")
                        .body(rendered)),
                    Err(e) => Ok(HttpResponse::InternalServerError()
                        .content_type("text/html; charset=utf-8")
                        .body(format!("Template Error: {}", e))),
                }
            })
        }
    }
}

#[cfg(not(feature = "async"))]
/// Placeholder when web framework features are not enabled
pub struct WebFrameworkPlaceholder;