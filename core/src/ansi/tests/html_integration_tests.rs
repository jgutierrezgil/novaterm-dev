use super::*;

#[test]
fn test_full_html_conversion_flow() {
    let mut converter = HtmlConverter::new("term".to_string());
    
    // Test sequence: Red text "Hello" followed by Blue text "World"
    let elements = vec![
        // Red color
        AnsiElement::Csi {
            params: vec![38, 2, 255, 0, 0],
            intermediates: vec![],
            ignore: false,
        },
        // Text
        AnsiElement::Text("Hello".to_string()),
        // Reset
        AnsiElement::Csi {
            params: vec![0],
            intermediates: vec![],
            ignore: false,
        },
        // Blue color
        AnsiElement::Csi {
            params: vec![38, 2, 0, 0, 255],
            intermediates: vec![],
            ignore: false,
        },
        // Text
        AnsiElement::Text("World".to_string()),
        // Reset
        AnsiElement::Csi {
            params: vec![0],
            intermediates: vec![],
            ignore: false,
        },
    ];
    
    // Convert each element
    let html: String = elements
        .iter()
        .map(|elem| converter.convert(elem).unwrap())
        .collect();
    
    // Get generated CSS
    let css = converter.get_css();
    
    // Verify HTML output
    assert_eq!(
        html,
        r#"<span class="term-rgb-255-0-0">Hello</span><span class="term-rgb-0-0-255">World</span>"#
    );
    
    // Verify CSS contains both color classes
    assert!(css.contains(".term-rgb-255-0-0 { color: rgb(255,0,0) }"));
    assert!(css.contains(".term-rgb-0-0-255 { color: rgb(0,0,255) }"));
}

#[test]
fn test_xss_prevention_with_formatting() {
    let mut converter = HtmlConverter::new("term".to_string());
    
    let elements = vec![
        // Red color
        AnsiElement::Csi {
            params: vec![38, 2, 255, 0, 0],
            intermediates: vec![],
            ignore: false,
        },
        // Malicious text
        AnsiElement::Text("<script>alert('xss');</script>".to_string()),
        // Reset
        AnsiElement::Csi {
            params: vec![0],
            intermediates: vec![],
            ignore: false,
        },
    ];
    
    let html: String = elements
        .iter()
        .map(|elem| converter.convert(elem).unwrap())
        .collect();
    
    assert_eq!(
        html,
        r#"<span class="term-rgb-255-0-0">&lt;script&gt;alert(&#x27;xss&#x27;);&lt;/script&gt;</span>"#
    );
}

#[test]
fn test_multiple_rgb_colors() {
    let mut converter = HtmlConverter::new("term".to_string());
    
    // Generate a rainbow text sequence
    let colors = vec![(255,0,0), (255,165,0), (255,255,0), (0,255,0), (0,0,255), (128,0,128)];
    let mut elements = Vec::new();
    
    for (r,g,b) in colors {
        // Add color
        elements.push(AnsiElement::Csi {
            params: vec![38, 2, r as i32, g as i32, b as i32],
            intermediates: vec![],
            ignore: false,
        });
        // Add text
        elements.push(AnsiElement::Text("■".to_string()));
        // Reset
        elements.push(AnsiElement::Csi {
            params: vec![0],
            intermediates: vec![],
            ignore: false,
        });
    }
    
    let html: String = elements
        .iter()
        .map(|elem| converter.convert(elem).unwrap())
        .collect();
    
    // Verify each color block is properly wrapped
    for (r,g,b) in colors {
        assert!(html.contains(&format!(
            r#"<span class="term-rgb-{}-{}-{}">"#,
            r, g, b
        )));
    }
    
    // Verify CSS contains all colors
    let css = converter.get_css();
    for (r,g,b) in colors {
        assert!(css.contains(&format!(
            ".term-rgb-{}-{}-{} {{ color: rgb({},{},{}) }}",
            r, g, b, r, g, b
        )));
    }
}
