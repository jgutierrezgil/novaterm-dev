use super::AnsiElement;
use std::collections::HashMap;
use std::fmt::Write;

/// Converts ANSI elements to HTML with CSS classes
pub struct HtmlConverter {
    class_prefix: String,
    style_cache: HashMap<String, String>,
}

#[derive(Debug, thiserror::Error)]
pub enum HtmlError {
    #[error("Invalid RGB parameters")]
    InvalidRgb,
    #[error("Format error: {0}")]
    Format(#[from] std::fmt::Error),
}

impl HtmlConverter {
    /// Creates a new HTML converter with the given class prefix
    pub fn new(class_prefix: String) -> Self {
        Self {
            class_prefix,
            style_cache: HashMap::new(),
        }
    }

    /// Converts an AnsiElement to HTML
    pub fn convert(&mut self, element: &AnsiElement) -> Result<String, HtmlError> {
        match element {
            AnsiElement::Text(text) => Ok(escape_html(text)),
            AnsiElement::Csi { params, .. } => self.convert_csi(params),
            _ => Ok(String::new()), // Ignore other elements for now
        }
    }

    /// Gets all generated CSS classes
    pub fn get_css(&self) -> String {
        let mut css = String::new();
        for (class, style) in &self.style_cache {
            let _ = writeln!(css, ".{} {{ {} }}", class, style);
        }
        css
    }

    fn convert_csi(&mut self, params: &[i32]) -> Result<String, HtmlError> {
        // Handle RGB color (38;2;r;g;b)
        if params.len() >= 5 && params[0] == 38 && params[1] == 2 {
            let (r, g, b) = (params[2], params[3], params[4]);
            if !(0..=255).contains(&r) || !(0..=255).contains(&g) || !(0..=255).contains(&b) {
                return Err(HtmlError::InvalidRgb);
            }
            
            let class = format!("{}-rgb-{}-{}-{}", self.class_prefix, r, g, b);
            let style = format!("color: rgb({},{},{})", r, g, b);
            
            // Cache the CSS class if not seen before
            if !self.style_cache.contains_key(&class) {
                self.style_cache.insert(class.clone(), style);
            }
            
            Ok(format!(r#"<span class="{}">"#, class))
        } else if params == [0] {
            // Reset formatting
            Ok("</span>".to_string())
        } else {
            Ok(String::new())
        }
    }
}

/// Escapes HTML special characters to prevent XSS
fn escape_html(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '&' => "&amp;".to_string(),
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&#x27;".to_string(),
            _ => c.to_string(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ansi::AnsiElement;

    #[test]
    fn test_rgb_color() {
        let mut converter = HtmlConverter::new("term".to_string());
        let element = AnsiElement::Csi {
            params: vec![38, 2, 255, 0, 0],
            intermediates: vec![],
            ignore: false,
        };
        
        assert_eq!(
            converter.convert(&element).unwrap(),
            r#"<span class="term-rgb-255-0-0">"#
        );
        
        assert_eq!(
            converter.get_css(),
            ".term-rgb-255-0-0 { color: rgb(255,0,0) }\n"
        );
    }

    #[test]
    fn test_invalid_rgb() {
        let mut converter = HtmlConverter::new("term".to_string());
        let element = AnsiElement::Csi {
            params: vec![38, 2, 300, 0, 0],
            intermediates: vec![],
            ignore: false,
        };
        
        assert!(matches!(converter.convert(&element), Err(HtmlError::InvalidRgb)));
    }

    #[test]
    fn test_xss_prevention() {
        let mut converter = HtmlConverter::new("term".to_string());
        let element = AnsiElement::Text("<script>alert('xss')</script>".to_string());
        
        assert_eq!(
            converter.convert(&element).unwrap(),
            "&lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt;"
        );
    }

    #[test]
    fn test_mixed_formatting() {
        let mut converter = HtmlConverter::new("term".to_string());
        
        // Red text
        let red = AnsiElement::Csi {
            params: vec![38, 2, 255, 0, 0],
            intermediates: vec![],
            ignore: false,
        };
        
        // Text content
        let text = AnsiElement::Text("Hello".to_string());
        
        // Reset
        let reset = AnsiElement::Csi {
            params: vec![0],
            intermediates: vec![],
            ignore: false,
        };
        
        assert_eq!(
            converter.convert(&red).unwrap() + 
            &converter.convert(&text).unwrap() + 
            &converter.convert(&reset).unwrap(),
            r#"<span class="term-rgb-255-0-0">Hello</span>"#
        );
    }
}
