//! Ecosystem compatibility and feature detection for v0.5.0

use crate::{TemplateResult, TemplateError};

/// Ecosystem compatibility information
#[derive(Debug, Clone)]
pub struct EcosystemCompatibility {
    pub async_supported: bool,
    pub wasm_compatible: bool,
    pub web_frameworks: Vec<String>,
    pub cli_tools_available: bool,
    pub deprecated_features: Vec<String>,
}

impl EcosystemCompatibility {
    /// Check current ecosystem compatibility
    #[allow(unused_mut)] // Mut needed for conditional compilation
    pub fn check() -> Self {
        let mut compatibility = EcosystemCompatibility {
            async_supported: false,
            wasm_compatible: false,
            web_frameworks: Vec::new(),
            cli_tools_available: false,
            deprecated_features: Vec::new(),
        };
        
        // Check async support
        #[cfg(feature = "async")]
        {
            compatibility.async_supported = true;
        }
        
        // Check WASM compatibility
        #[cfg(feature = "wasm")]
        {
            compatibility.wasm_compatible = true;
        }
        
        // Check web framework integrations
        #[cfg(feature = "axum-integration")]
        {
            compatibility.web_frameworks.push("axum".to_string());
        }
        
        #[cfg(feature = "warp-integration")]
        {
            compatibility.web_frameworks.push("warp".to_string());
        }
        
        #[cfg(feature = "actix-integration")]
        {
            compatibility.web_frameworks.push("actix-web".to_string());
        }
        
        // Check CLI tools
        #[cfg(feature = "cli")]
        {
            compatibility.cli_tools_available = true;
        }
        
        compatibility
    }
}

/// Extension trait to add ecosystem compatibility to TemplateEngine
pub trait EcosystemTemplateEngine {
    /// Check ecosystem compatibility
    fn check_ecosystem_compatibility(&self) -> TemplateResult<EcosystemCompatibility>;
    
    /// Try an async operation (returns error if not available)
    fn try_async_operation(&self) -> TemplateResult<()>;
}

impl EcosystemTemplateEngine for crate::TemplateEngine {
    fn check_ecosystem_compatibility(&self) -> TemplateResult<EcosystemCompatibility> {
        Ok(EcosystemCompatibility::check())
    }
    
    fn try_async_operation(&self) -> TemplateResult<()> {
        #[cfg(feature = "async")]
        {
            Ok(())
        }
        
        #[cfg(not(feature = "async"))]
        {
            Err(TemplateError::Runtime(
                "async feature not enabled. Add 'async' feature to Cargo.toml dependencies".to_string()
            ))
        }
    }
}