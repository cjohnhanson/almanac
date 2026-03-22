//! Bundled documentation, baked in at compile time.

/// A single documentation page.
pub struct DocPage {
    pub slug: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub raw: &'static str,
}

impl DocPage {
    /// Return the markdown content with the metadata comment stripped.
    pub fn content(&self) -> &str {
        let md = self.raw;
        if let Some(start) = md.find("<!-- metadata") {
            if let Some(end) = md[start..].find("-->") {
                return md[start + end + 3..].trim_start_matches('\n');
            }
        }
        md
    }
}

/// All almanac documentation pages.
pub static PAGES: &[DocPage] = &[
    DocPage {
        slug: "what-is-almanac",
        title: "What is Almanac?",
        description: "Agent skill aggregation from pluggable sources",
        raw: include_str!("../docs/what-is-almanac.md"),
    },
    DocPage {
        slug: "cli-reference",
        title: "CLI Reference",
        description: "Complete command reference for the almanac skill aggregator",
        raw: include_str!("../docs/cli-reference.md"),
    },
];

/// Print a listing of all docs to stdout.
pub fn list() {
    for page in PAGES {
        println!("{:<25} {}", page.title, page.description);
    }
}

/// Print a doc by slug. Returns false if not found.
pub fn show(slug: &str) -> bool {
    if let Some(page) = PAGES.iter().find(|p| p.slug == slug) {
        print!("{}", page.content());
        true
    } else {
        false
    }
}

/// Search docs for a query string. Prints matching doc titles.
pub fn search(query: &str) {
    let q = query.to_lowercase();
    for page in PAGES {
        if page.title.to_lowercase().contains(&q)
            || page.description.to_lowercase().contains(&q)
            || page.content().to_lowercase().contains(&q)
        {
            println!("{:<25} {}", page.title, page.description);
        }
    }
}
