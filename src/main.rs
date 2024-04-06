use pulldown_cmark::{Options, Parser};
// This is a simple hello world program in Rust
fn main() {
    let markdown_input = "Hello world, this is a ~~complicated~~ *very simple* example.";

    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown_input, options);

    // Write to String buffer.
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    // Check that the output is what we expected.
    let expected_html =
        "<p>Hello world, this is a <del>complicated</del> <em>very simple</em> example.</p>\n";
    assert_eq!(expected_html, &html_output);
}
