use pulldown_cmark::{html, Parser};

// Convert Markdown string into a HTML string
pub fn convert_md_to_html(md_input: &str) -> String {

    let parser = Parser::new(md_input);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}