//! Async template rendering support for v0.5.0
//!
//! Provides non-blocking template rendering capabilities for high-performance web applications

#[cfg(feature = "async")]
use crate::{TemplateEngine, TemplateContext, TemplateResult, TemplateError};
#[cfg(feature = "async")]
use std::path::Path;
#[cfg(feature = "async")]
use tokio::fs;
#[cfg(feature = "async")]
use futures::future::BoxFuture;

#[cfg(feature = "async")]
/// Async extension trait for TemplateEngine
pub trait AsyncTemplateEngine {
    /// Render a template string asynchronously
    fn render_string_async<'a>(&'a mut self, template: &'a str, context: &'a TemplateContext) -> BoxFuture<'a, TemplateResult<String>>;
    
    /// Render a template file asynchronously
    fn render_async<'a>(&'a mut self, template_name: &'a str, context: &'a TemplateContext) -> BoxFuture<'a, TemplateResult<String>>;
    
    /// Load a template file asynchronously
    fn load_template_async<'a>(&'a self, template_name: &'a str) -> BoxFuture<'a, TemplateResult<String>>;
    
    /// Clone the engine for concurrent use
    fn clone(&self) -> Self;
}

#[cfg(feature = "async")]
impl AsyncTemplateEngine for TemplateEngine {
    fn render_string_async<'a>(&'a mut self, template: &'a str, context: &'a TemplateContext) -> BoxFuture<'a, TemplateResult<String>> {
        Box::pin(async move {
            // For now, we'll use the sync version wrapped in async
            // TODO: Implement true async template parsing and rendering
            tokio::task::yield_now().await; // Yield to allow other tasks
            self.render_string(template, context)
        })
    }

    fn render_async<'a>(&'a mut self, template_name: &'a str, context: &'a TemplateContext) -> BoxFuture<'a, TemplateResult<String>> {
        Box::pin(async move {
            // Load template asynchronously
            let template_content = self.load_template_async(template_name).await?;
            
            // Render asynchronously
            self.render_string_async(&template_content, context).await
        })
    }

    fn load_template_async<'a>(&'a self, template_name: &'a str) -> BoxFuture<'a, TemplateResult<String>> {
        let template_path = Path::new(self.get_template_dir()).join(template_name);
        
        Box::pin(async move {
            match fs::read_to_string(&template_path).await {
                Ok(content) => Ok(content),
                Err(e) => Err(TemplateError::Io(e)),
            }
        })
    }

    fn clone(&self) -> Self {
        // Use the existing Clone implementation from the struct
        Clone::clone(self)
    }
}


#[cfg(not(feature = "async"))]
/// Placeholder when async feature is not enabled
#[allow(dead_code)]
pub struct AsyncPlaceholder;

#[cfg(not(feature = "async"))]
impl AsyncPlaceholder {
    #[allow(dead_code)]
    pub fn try_async_operation() -> Result<(), crate::TemplateError> {
        Err(crate::TemplateError::Runtime("async feature not enabled. Enable the 'async' feature in Cargo.toml".to_string()))
    }
}