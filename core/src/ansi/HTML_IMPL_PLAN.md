# HTML Conversion Implementation Plan

## 1. New Module Structure
```rust
// html.rs
pub struct HtmlConverter {
    class_prefix: String,
    style_cache: HashMap<String, String>,
}
```

## 2. Color Handling
- Parse CSI sequences: `38;2;r;g;b` for RGB colors
- Generate CSS classes for each unique color
- Class naming scheme: `{prefix}-rgb-{r}-{g}-{b}`

## 3. HTML Generation
- Convert AnsiElement to HTML spans with appropriate classes
- Escape special characters for XSS prevention
- Cache generated CSS classes for performance

## 4. CSS Class Generation
```css
.term-rgb-255-0-0 {
    color: rgb(255,0,0);
}
```

## 5. XSS Prevention
- Escape HTML special characters:
  - `&` -> `&amp;`
  - `<` -> `&lt;`
  - `>` -> `&gt;`
  - `"` -> `&quot;`
  - `'` -> `&#x27;`

## 6. Unit Tests
1. RGB Color Tests:
   - Basic RGB color conversion
   - Multiple color sequences
   - Invalid color parameters

2. HTML Generation Tests:
   - Text with no formatting
   - Text with RGB colors
   - Mixed formatting

3. XSS Prevention Tests:
   - HTML injection attempts
   - Script injection attempts
   - Special character handling

## 7. API Design
```rust
impl HtmlConverter {
    pub fn new(class_prefix: String) -> Self;
    pub fn convert(&mut self, element: &AnsiElement) -> String;
    pub fn get_css(&self) -> String;
}
```

## 8. Performance Considerations
- Cache generated CSS classes
- Minimize string allocations
- Reuse color classes when possible

## 9. Example Usage
```rust
let mut converter = HtmlConverter::new("term".to_string());
let html = converter.convert(&ansi_element);
let css = converter.get_css();
