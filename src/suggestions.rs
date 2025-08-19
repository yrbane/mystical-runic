//! Suggestion engine for helpful error messages
//! 
//! Provides smart suggestions for common mistakes and typos

use std::collections::HashMap;

/// Calculate Levenshtein distance between two strings
pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let a_len = a_chars.len();
    let b_len = b_chars.len();
    
    let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];
    
    // Initialize first row and column
    for i in 0..=a_len {
        matrix[i][0] = i;
    }
    for j in 0..=b_len {
        matrix[0][j] = j;
    }
    
    // Fill the matrix
    for i in 1..=a_len {
        for j in 1..=b_len {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            matrix[i][j] = std::cmp::min(
                std::cmp::min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1),
                matrix[i - 1][j - 1] + cost,
            );
        }
    }
    
    matrix[a_len][b_len]
}

/// Generate suggestions for a misspelled template name
pub fn suggest_templates(target: &str, available: &[String], max_suggestions: usize) -> Vec<String> {
    let mut suggestions_with_distance: Vec<(String, usize)> = available
        .iter()
        .map(|template| {
            let distance = levenshtein_distance(target, template);
            (template.clone(), distance)
        })
        .collect();
    
    // Sort by distance (best matches first)
    suggestions_with_distance.sort_by_key(|(_, distance)| *distance);
    
    // Filter out suggestions that are too different (distance > 50% of target length)
    let max_distance = (target.len() / 2).max(1);
    suggestions_with_distance.retain(|(_, distance)| *distance <= max_distance);
    
    // Return top suggestions
    suggestions_with_distance
        .into_iter()
        .take(max_suggestions)
        .map(|(template, _)| template)
        .collect()
}

/// Generate suggestions for a misspelled variable name
pub fn suggest_variables(target: &str, available: &[String], max_suggestions: usize) -> Vec<String> {
    let mut suggestions_with_score: Vec<(String, f64)> = available
        .iter()
        .map(|variable| {
            let distance = levenshtein_distance(target, variable);
            let max_len = target.len().max(variable.len());
            let similarity = if max_len == 0 { 1.0 } else { 1.0 - (distance as f64 / max_len as f64) };
            
            // Bonus for common prefixes
            let prefix_bonus = if variable.starts_with(&target[..target.len().min(3)]) { 0.2 } else { 0.0 };
            
            // Bonus for partial matches
            let partial_bonus = if variable.contains(target) || target.contains(variable) { 0.1 } else { 0.0 };
            
            (variable.clone(), similarity + prefix_bonus + partial_bonus)
        })
        .collect();
    
    // Sort by score (best matches first)
    suggestions_with_score.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    // Filter out suggestions that are too dissimilar (score < 0.3)
    suggestions_with_score.retain(|(_, score)| *score >= 0.3);
    
    // Return top suggestions
    suggestions_with_score
        .into_iter()
        .take(max_suggestions)
        .map(|(variable, _)| variable)
        .collect()
}

/// Extract context lines around an error location
pub fn extract_context_lines(content: &str, target_line: usize, context_size: usize) -> Vec<String> {
    let lines: Vec<&str> = content.lines().collect();
    let total_lines = lines.len();
    
    if total_lines == 0 || target_line == 0 {
        return Vec::new();
    }
    
    let target_index = target_line - 1; // Convert to 0-based index
    
    let start_index = target_index.saturating_sub(context_size);
    let end_index = (target_index + context_size + 1).min(total_lines);
    
    lines[start_index..end_index]
        .iter()
        .map(|line| line.to_string())
        .collect()
}

/// Find line and column position of a character offset in text
pub fn find_line_column(content: &str, char_offset: usize) -> (usize, usize) {
    let mut line = 1;
    let mut column = 1;
    
    for (i, ch) in content.char_indices() {
        if i >= char_offset {
            break;
        }
        
        if ch == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    
    (line, column)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("cat", "bat"), 1);
        assert_eq!(levenshtein_distance("user_profile", "user_profil"), 1);
        assert_eq!(levenshtein_distance("hello", "world"), 4);
        assert_eq!(levenshtein_distance("same", "same"), 0);
    }

    #[test]
    fn test_suggest_templates() {
        let available = vec![
            "user_profile.html".to_string(),
            "admin_panel.html".to_string(),
            "home.html".to_string(),
        ];
        
        let suggestions = suggest_templates("user_profil.html", &available, 3);
        assert!(!suggestions.is_empty());
        assert_eq!(suggestions[0], "user_profile.html");
    }

    #[test]
    fn test_suggest_variables() {
        let available = vec![
            "user_name".to_string(),
            "user_email".to_string(),
            "admin_level".to_string(),
        ];
        
        let suggestions = suggest_variables("username", &available, 3);
        assert!(!suggestions.is_empty());
        assert_eq!(suggestions[0], "user_name");
    }

    #[test]
    fn test_find_line_column() {
        let content = "Line 1\nLine 2\nLine 3";
        assert_eq!(find_line_column(content, 0), (1, 1));
        assert_eq!(find_line_column(content, 7), (2, 1));
        assert_eq!(find_line_column(content, 10), (2, 4));
    }

    #[test]
    fn test_extract_context_lines() {
        let content = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
        let context = extract_context_lines(content, 3, 1);
        assert_eq!(context.len(), 3);
        assert_eq!(context[0], "Line 2");
        assert_eq!(context[1], "Line 3");
        assert_eq!(context[2], "Line 4");
    }
}