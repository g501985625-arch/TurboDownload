//! HTML parser module

use scraper::{Html, Selector, ElementRef};

/// HTML parser for extracting content
pub struct HtmlParser {
    document: Html,
}

impl HtmlParser {
    /// Parse HTML string
    pub fn new(html: &str) -> Self {
        Self {
            document: Html::parse_document(html),
        }
    }
    
    /// Select elements by CSS selector
    pub fn select(&self, selector: &str) -> Vec<ElementRef<'_>> {
        let selector = match Selector::parse(selector) {
            Ok(s) => s,
            Err(_) => return vec![],
        };
        
        self.document.select(&selector).collect()
    }
    
    /// Get text content of an element
    pub fn text(&self, element: &ElementRef) -> String {
        element.text().collect::<Vec<_>>().join("")
    }
    
    /// Get attribute value
    pub fn attr(&self, element: &ElementRef, attr: &str) -> Option<String> {
        element.value().attr(attr).map(String::from)
    }
    
    /// Extract all links (href attributes)
    pub fn extract_links(&self) -> Vec<String> {
        let selector = Selector::parse("a[href]").unwrap();
        
        self.document.select(&selector)
            .filter_map(|el| el.value().attr("href"))
            .map(String::from)
            .collect()
    }
    
    /// Extract all images (src attributes)
    pub fn extract_images(&self) -> Vec<String> {
        let selector = Selector::parse("img[src]").unwrap();
        
        self.document.select(&selector)
            .filter_map(|el| el.value().attr("src"))
            .map(String::from)
            .collect()
    }
    
    /// Extract all scripts (src attributes)
    pub fn extract_scripts(&self) -> Vec<String> {
        let selector = Selector::parse("script[src]").unwrap();
        
        self.document.select(&selector)
            .filter_map(|el| el.value().attr("src"))
            .map(String::from)
            .collect()
    }
    
    /// Extract all stylesheets (href attributes)
    pub fn extract_stylesheets(&self) -> Vec<String> {
        let selector = Selector::parse("link[rel='stylesheet'][href]").unwrap();
        
        self.document.select(&selector)
            .filter_map(|el| el.value().attr("href"))
            .map(String::from)
            .collect()
    }
    
    /// Get page title
    pub fn title(&self) -> Option<String> {
        let selector = Selector::parse("title").ok()?;
        self.document.select(&selector).next().map(|el| el.text().collect())
    }
    
    /// Get all meta tags
    pub fn meta_tags(&self) -> Vec<(String, String)> {
        let selector = Selector::parse("meta").unwrap();
        
        self.document.select(&selector)
            .filter_map(|el| {
                let name = el.value().attr("name").or(el.value().attr("property"));
                let content = el.value().attr("content");
                match (name, content) {
                    (Some(n), Some(c)) => Some((n.to_string(), c.to_string())),
                    _ => None,
                }
            })
            .collect()
    }
}