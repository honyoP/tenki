use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use regex::Regex;
use std::sync::LazyLock;

static WIKILINK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[\[([^\]|]+)(?:\|[^\]]+)?\]\]").unwrap());

/// Extract all [[wikilinks]] from markdown content.
/// Supports both [[link]] and [[link|display text]] syntax.
pub fn extract_wikilinks(content: &str) -> Vec<String> {
    WIKILINK_RE
        .captures_iter(content)
        .map(|cap| cap[1].to_string())
        .collect()
}

/// Parsed representation of a markdown document.
#[derive(Debug, Default)]
pub struct ParsedNote {
    pub title: Option<String>,
    pub headings: Vec<String>,
    pub links: Vec<String>,
    pub wikilinks: Vec<String>,
}

/// Parse markdown content and extract structure.
pub fn parse_markdown(content: &str) -> ParsedNote {
    let parser = Parser::new(content);
    let mut note = ParsedNote::default();
    let mut in_heading = false;
    let mut heading_text = String::new();
    let mut first_heading = true;

    for event in parser {
        match event {
            Event::Start(Tag::Heading { .. }) => {
                in_heading = true;
                heading_text.clear();
            }
            Event::End(TagEnd::Heading(_)) => {
                in_heading = false;
                if first_heading && note.title.is_none() {
                    note.title = Some(heading_text.clone());
                    first_heading = false;
                }
                note.headings.push(heading_text.clone());
            }
            Event::Text(text) => {
                if in_heading {
                    heading_text.push_str(&text);
                }
            }
            Event::Start(Tag::Link { dest_url, .. }) => {
                note.links.push(dest_url.to_string());
            }
            _ => {}
        }
    }

    note.wikilinks = extract_wikilinks(content);
    note
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_wikilinks() {
        let content = "This is a [[test]] with [[multiple|links]] inside.";
        let links = extract_wikilinks(content);
        assert_eq!(links, vec!["test", "multiple"]);
    }

    #[test]
    fn test_parse_markdown() {
        let content = "# My Note\n\nSome text with [[wikilink]].\n\n## Section\n\nMore text.";
        let parsed = parse_markdown(content);
        assert_eq!(parsed.title, Some("My Note".to_string()));
        assert_eq!(parsed.headings.len(), 2);
        assert_eq!(parsed.wikilinks, vec!["wikilink"]);
    }
}
