//! Template layouts and inheritance system for v0.2.0

use crate::error::{TemplateError, TemplateResult};
use crate::context::TemplateContext;
use std::collections::HashMap;

/// Block definition for template inheritance
#[derive(Debug, Clone)]
pub struct Block {
    pub name: String,
    pub content: String,
    pub parent_content: Option<String>, // For {{super}} functionality
}

/// Layout inheritance information
#[derive(Debug, Clone)]
pub struct LayoutInfo {
    pub extends: Option<String>, // Parent template name
    pub blocks: HashMap<String, Block>,
    pub content: String,
}

/// Template layout processor
#[derive(Debug, Clone)]
pub struct LayoutProcessor {
    pub templates: HashMap<String, LayoutInfo>,
}

impl LayoutProcessor {
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
        }
    }
    
    /// Parse template and extract layout information
    pub fn parse_template(&mut self, name: &str, content: &str) -> TemplateResult<LayoutInfo> {
        let mut layout_info = LayoutInfo {
            extends: None,
            blocks: HashMap::new(),
            content: content.to_string(),
        };
        
        // Check for {{extends}} directive
        if let Some(extends_match) = self.find_extends_directive(content) {
            layout_info.extends = Some(extends_match);
            // Remove extends directive from content
            layout_info.content = self.remove_extends_directive(content);
        }
        
        // Extract blocks from template
        layout_info.blocks = self.extract_blocks(&layout_info.content)?;
        
        // Cache the layout info
        self.templates.insert(name.to_string(), layout_info.clone());
        
        Ok(layout_info)
    }
    
    /// Resolve inheritance chain and merge blocks
    pub fn resolve_inheritance(&self, template_name: &str) -> TemplateResult<String> {
        let layout_info = self.templates.get(template_name)
            .ok_or_else(|| TemplateError::Template(format!("Template '{}' not found", template_name)))?;
        
        
        if let Some(parent_name) = &layout_info.extends {
            // Get parent template
            let parent_layout = self.templates.get(parent_name)
                .ok_or_else(|| TemplateError::Template(format!("Parent template '{}' not found", parent_name)))?;
            
            // Start with parent template content
            let mut resolved_content = parent_layout.content.clone();
            
            // Recursively resolve parent inheritance
            if parent_layout.extends.is_some() {
                resolved_content = self.resolve_inheritance(parent_name)?;
            }
            
            // Replace blocks in parent with child blocks
            resolved_content = self.merge_blocks(&resolved_content, &layout_info.blocks, &parent_layout.blocks)?;
            
            Ok(resolved_content)
        } else {
            // No inheritance, just replace blocks with their content
            self.merge_blocks(&layout_info.content, &layout_info.blocks, &HashMap::new())
        }
    }
    
    /// Find {{extends "template"}} directive
    fn find_extends_directive(&self, content: &str) -> Option<String> {
        if let Some(start) = content.find("{{extends ") {
            if let Some(end) = content[start..].find("}}") {
                let directive = &content[start + 10..start + end];
                let template_name = directive.trim().trim_matches('"').trim_matches('\'');
                return Some(template_name.to_string());
            }
        }
        None
    }
    
    /// Remove {{extends}} directive from content
    fn remove_extends_directive(&self, content: &str) -> String {
        if let Some(start) = content.find("{{extends ") {
            if let Some(end) = content[start..].find("}}") {
                let mut result = content[..start].to_string();
                result.push_str(&content[start + end + 2..]);
                return result.trim().to_string();
            }
        }
        content.to_string()
    }
    
    /// Extract {{block name}}...{{/block}} definitions
    fn extract_blocks(&self, content: &str) -> TemplateResult<HashMap<String, Block>> {
        let mut blocks = HashMap::new();
        let mut pos = 0;
        
        while let Some(block_start) = content[pos..].find("{{block ") {
            let absolute_start = pos + block_start;
            
            // Find end of opening tag
            let tag_end = content[absolute_start..].find("}}")
                .ok_or_else(|| TemplateError::Parse("Unclosed block directive".to_string()))?;
            
            let block_name = content[absolute_start + 8..absolute_start + tag_end].trim();
            let content_start = absolute_start + tag_end + 2;
            
            // Find matching {{/block}}
            let block_end = self.find_matching_block_end(&content[content_start..], block_name)?;
            let absolute_end = content_start + block_end;
            
            let block_content = content[content_start..absolute_end].trim().to_string();
            
            blocks.insert(block_name.to_string(), Block {
                name: block_name.to_string(),
                content: block_content,
                parent_content: None,
            });
            
            // Calculate how far to skip - need to skip past the entire {{/block}} tag
            let end_tag_len = if content[absolute_end..].starts_with(&format!("{{{{/block {}}}}}", block_name)) {
                block_name.len() + 10 // {{/block name}}
            } else {
                9 // {{/block}}
            };
            pos = absolute_end + end_tag_len;
        }
        Ok(blocks)
    }
    
    /// Find matching {{/block}} for a given block, handling nested blocks properly
    fn find_matching_block_end(&self, content: &str, block_name: &str) -> TemplateResult<usize> {
        let end_tag = format!("{{{{/block {}}}}}", block_name);
        let simple_end_tag = "{{/block}}";
        
        let mut pos = 0;
        let mut nesting_level = 1; // We're already inside one block
        
        while pos < content.len() {
            // Look for either an opening or closing block
            let remaining = &content[pos..];
            
            if let Some(open_pos) = remaining.find("{{block ") {
                if let Some(close_pos) = remaining.find(simple_end_tag) {
                    let abs_open = pos + open_pos;
                    let abs_close = pos + close_pos;
                    
                    if abs_open < abs_close {
                        // Opening block comes first - increase nesting
                        nesting_level += 1;
                        pos = abs_open + 8; // Skip past "{{block "
                    } else {
                        // Closing block comes first - decrease nesting
                        nesting_level -= 1;
                        if nesting_level == 0 {
                            return Ok(abs_close);
                        }
                        pos = abs_close + simple_end_tag.len();
                    }
                } else if let Some(close_pos) = remaining.find(&end_tag) {
                    // Found specific closing tag
                    let abs_close = pos + close_pos;
                    nesting_level -= 1;
                    if nesting_level == 0 {
                        return Ok(abs_close);
                    }
                    pos = abs_close + end_tag.len();
                } else {
                    // Found opening but no closing - error
                    break;
                }
            } else if let Some(close_pos) = remaining.find(simple_end_tag) {
                // Only closing blocks remain
                let abs_close = pos + close_pos;
                nesting_level -= 1;
                if nesting_level == 0 {
                    return Ok(abs_close);
                }
                pos = abs_close + simple_end_tag.len();
            } else if let Some(close_pos) = remaining.find(&end_tag) {
                // Only specific closing blocks remain
                let abs_close = pos + close_pos;
                nesting_level -= 1;
                if nesting_level == 0 {
                    return Ok(abs_close);
                }
                pos = abs_close + end_tag.len();
            } else {
                // No more blocks found
                break;
            }
        }
        
        Err(TemplateError::Parse(format!("Missing {{{{/block}}}} for block '{}'", block_name)))
    }
    
    /// Merge child blocks with parent template
    fn merge_blocks(
        &self, 
        template_content: &str, 
        child_blocks: &HashMap<String, Block>,
        _parent_blocks: &HashMap<String, Block>
    ) -> TemplateResult<String> {
        let mut result = template_content.to_string();
        
        // Find and replace all blocks from end to start to avoid position invalidation
        let mut blocks_to_replace = Vec::new();
        let mut pos = 0;
        
        // First pass: find all block positions
        while let Some(block_start) = result[pos..].find("{{block ") {
            let absolute_start = pos + block_start;
            
            // Find end of opening tag
            let tag_end = result[absolute_start..].find("}}")
                .ok_or_else(|| TemplateError::Parse("Unclosed block directive".to_string()))?;
            
            let block_directive = &result[absolute_start + 8..absolute_start + tag_end];
            let parts: Vec<&str> = block_directive.split_whitespace().collect();
            let block_name = parts[0];
            
            // Find content between opening and closing tags
            let content_start = absolute_start + tag_end + 2;
            let block_content_end = self.find_matching_block_end(&result[content_start..], block_name)?;
            let absolute_content_end = content_start + block_content_end;
            let default_content = result[content_start..absolute_content_end].trim().to_string();
            
            // Find end of closing tag - absolute_content_end points to START of closing tag
            let closing_tag_text = &result[absolute_content_end..];
            let end_tag_end = if closing_tag_text.starts_with(&format!("{{{{/block {}}}}}", block_name)) {
                absolute_content_end + format!("{{{{/block {}}}}}", block_name).len()
            } else if closing_tag_text.starts_with("{{/block}}") {
                absolute_content_end + "{{/block}}".len()
            } else {
                return Err(TemplateError::Parse(format!(
                    "Invalid block end position for '{}'. Found: '{}'", 
                    block_name, 
                    &closing_tag_text[..closing_tag_text.len().min(20)]
                )));
            };
            
            // Determine replacement content
            let replacement_content = if let Some(child_block) = child_blocks.get(block_name) {
                // Use child block content, process {{super}} if present
                self.process_super_directive(&child_block.content, &default_content)
            } else {
                // Use default content from parent template
                default_content
            };
            
            blocks_to_replace.push((absolute_start, end_tag_end, replacement_content));
            pos = end_tag_end;
        }
        
        // Second pass: replace blocks from end to start
        blocks_to_replace.reverse();
        for (start, end, replacement) in blocks_to_replace {
            result.replace_range(start..end, &replacement);
        }
        
        Ok(result)
    }
    
    /// Process {{super}} directive in block content
    fn process_super_directive(&self, content: &str, parent_content: &str) -> String {
        content.replace("{{super}}", parent_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_simple_template() {
        let mut processor = LayoutProcessor::new();
        let content = r#"
        {{block title}}Default Title{{/block}}
        <h1>{{name}}</h1>
        "#;
        
        let layout = processor.parse_template("test.html", content).unwrap();
        assert!(layout.extends.is_none());
        assert_eq!(layout.blocks.len(), 1);
        assert!(layout.blocks.contains_key("title"));
    }
    
    #[test]
    fn test_parse_template_with_extends() {
        let mut processor = LayoutProcessor::new();
        let content = r#"
        {{extends "base.html"}}
        {{block content}}Child content{{/block}}
        "#;
        
        let layout = processor.parse_template("child.html", content).unwrap();
        assert_eq!(layout.extends, Some("base.html".to_string()));
        assert_eq!(layout.blocks.len(), 1);
        assert!(layout.blocks.contains_key("content"));
    }
    
    #[test]
    fn test_super_directive() {
        let processor = LayoutProcessor::new();
        let child_content = "{{super}} and additional content";
        let parent_content = "Parent content";
        
        let result = processor.process_super_directive(child_content, parent_content);
        assert_eq!(result, "Parent content and additional content");
    }
}