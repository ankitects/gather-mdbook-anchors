use std::collections::HashMap;
use std::env;

use mdbook::BookItem;
use mdbook::{errors::Error, MDBook};
use regex::Regex;

/// Modified version of mdbook::renderer::html_handlebars::hbs_renderer::build_header_links;
pub fn build_header_links(html: &str) -> Vec<String> {
    let regex = Regex::new(r"<h(\d)>(.*?)</h\d>").unwrap();
    let mut id_counter = HashMap::new();
    let mut out = vec![];

    for caps in regex.captures_iter(html) {
        out.push(get_link_id(&caps[2], &mut id_counter))
    }

    out
}

/// Modified version of mdbook::renderer::html_handlebars::hbs_renderer::insert_link_into_header;
fn get_link_id(content: &str, id_counter: &mut HashMap<String, usize>) -> String {
    let raw_id = mdbook::utils::id_from_content(content);

    let id_count = id_counter.entry(raw_id.clone()).or_insert(0);

    let id = match *id_count {
        0 => raw_id,
        other => format!("{}-{}", raw_id, other),
    };

    *id_count += 1;

    id
}

fn main() -> Result<(), Error> {
    let args: Vec<_> = env::args().collect();
    let book = MDBook::load(&args[1])?;
    for item in book.iter() {
        if let BookItem::Chapter(chapter) = item {
            let markdown = mdbook::utils::render_markdown_with_path(
                &chapter.content,
                false,
                Some(chapter.path.as_ref().unwrap()),
            );
            let path = chapter.path.as_ref().unwrap().to_string_lossy();
            for link in build_header_links(&markdown) {
                println!("{}#{}", path, link);
            }
        }
    }

    Ok(())
}
