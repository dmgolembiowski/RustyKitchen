#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
extern crate reqwest;
extern crate regex;

use std::io::Read;
use std::collections::HashSet;
use std::borrow::Cow;
use regex::Regex;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Reqwest(reqwest::Error);
        Regex(regex::Error);
    }
}

fn extract_links(content: &str) -> Result<HashSet<Cow<str>>> {
    lazy_static! {
        static ref WIKI_REGEX: Regex =
            Regex::new(r"(?x)
                \[\[(?P<internal>[^\[\]|]*)[^\[\]]*\]\]    # internal links
                |
                (url=|URL\||\[)(?P<external>http.*?)[ \|}] # external links
            ").unwrap();
    }

    let links: HashSet<_> = WIKI_REGEX
        .captures_iter(content)
        .map(|c| match (c.name("internal"), c.name("external")) {
            (Some(val), None) => Cow::from(val.as_str().to_lowercase()),
            (None, Some(val)) => Cow::from(val.as_str()),
            _ => unreachable!(),
        })
        .collect();

    Ok(links)
}

fn run() -> Result<()> {
    let mut content = String::new();
    reqwest::get(
        "https://en.wikipedia.org/w/index.php?title=Rust_(programming_language)&action=raw",
    )?
        .read_to_string(&mut content)?;

    println!("{:#?}", extract_links(&content)?);

    Ok(())
}

quick_main!(run);

