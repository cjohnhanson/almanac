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
    print!("{}", format_list(PAGES));
}

/// Print a doc by identifier. Returns false if not found.
pub fn show(identifier: &str) -> bool {
    if let Some(page) = find(identifier) {
        print!("{}", page.content());
        true
    } else {
        false
    }
}

/// Search docs for a query string. Prints matching doc titles.
pub fn search(query: &str) {
    let matches = find_matching(PAGES, query);
    if matches.is_empty() {
        eprintln!("no docs matching '{query}'");
    } else {
        print!("{}", format_list_from_refs(&matches));
    }
}

/// Find a doc page by flexible identifier: exact slug, slug prefix, or
/// case-insensitive title match.
pub fn find(identifier: &str) -> Option<&'static DocPage> {
    find_in(PAGES, identifier)
}

/// Find a doc page in a given slice by flexible identifier.
pub fn find_in<'a>(pages: &'a [DocPage], identifier: &str) -> Option<&'a DocPage> {
    // 1. Exact slug match
    if let Some(page) = pages.iter().find(|p| p.slug == identifier) {
        return Some(page);
    }
    // 2. Case-insensitive slug match
    let lower = identifier.to_lowercase();
    if let Some(page) = pages.iter().find(|p| p.slug.to_lowercase() == lower) {
        return Some(page);
    }
    // 3. Case-insensitive title match
    if let Some(page) = pages.iter().find(|p| p.title.to_lowercase() == lower) {
        return Some(page);
    }
    // 4. Unique slug prefix
    let prefix_matches: Vec<_> = pages
        .iter()
        .filter(|p| p.slug.starts_with(identifier) || p.slug.starts_with(&lower))
        .collect();
    if prefix_matches.len() == 1 {
        return Some(prefix_matches[0]);
    }
    None
}

/// Format a listing of doc pages showing slug (what to type) and description.
pub fn format_list(pages: &[DocPage]) -> String {
    let mut out = String::new();
    for page in pages {
        out.push_str(&format!("  {:<25} {}\n", page.slug, page.description));
    }
    out
}

/// Format a listing from a vec of page references.
pub fn format_list_from_refs(pages: &[&DocPage]) -> String {
    let mut out = String::new();
    for page in pages {
        out.push_str(&format!("  {:<25} {}\n", page.slug, page.description));
    }
    out
}

/// Find all docs matching a query string. Returns matching pages.
pub fn find_matching<'a>(pages: &'a [DocPage], query: &str) -> Vec<&'a DocPage> {
    let q = query.to_lowercase();
    pages
        .iter()
        .filter(|page| {
            page.slug.to_lowercase().contains(&q)
                || page.title.to_lowercase().contains(&q)
                || page.description.to_lowercase().contains(&q)
                || page.content().to_lowercase().contains(&q)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_PAGES: &[DocPage] = &[
        DocPage {
            slug: "what-is-almanac",
            title: "What is Almanac?",
            description: "Agent skill aggregation from pluggable sources",
            raw: "<!-- metadata\ntitle: \"What is Almanac?\"\n-->\n# What is Almanac?\nAlmanac aggregates skills.",
        },
        DocPage {
            slug: "cli-reference",
            title: "CLI Reference",
            description: "Complete command reference for the almanac skill aggregator",
            raw: "<!-- metadata\ntitle: \"CLI Reference\"\n-->\n# CLI Reference\nUsage details.",
        },
        DocPage {
            slug: "cli-advanced",
            title: "Advanced CLI",
            description: "Advanced patterns and scripting",
            raw: "Advanced usage patterns for power users.",
        },
    ];

    // --- content() metadata stripping ---

    #[test]
    fn content_strips_metadata_comment() {
        let page = &TEST_PAGES[0];
        let content = page.content();
        assert!(!content.contains("<!-- metadata"), "metadata comment should be stripped");
        assert!(content.starts_with("# What is Almanac?"));
    }

    #[test]
    fn content_returns_raw_when_no_metadata() {
        let page = &TEST_PAGES[2];
        assert_eq!(page.content(), page.raw);
    }

    // --- find_in: exact slug ---

    #[test]
    fn find_exact_slug() {
        let page = find_in(TEST_PAGES, "what-is-almanac").unwrap();
        assert_eq!(page.slug, "what-is-almanac");
    }

    #[test]
    fn find_exact_slug_other() {
        let page = find_in(TEST_PAGES, "cli-reference").unwrap();
        assert_eq!(page.slug, "cli-reference");
    }

    // --- find_in: case-insensitive slug ---

    #[test]
    fn find_slug_case_insensitive() {
        let page = find_in(TEST_PAGES, "What-Is-Almanac").unwrap();
        assert_eq!(page.slug, "what-is-almanac");
    }

    // --- find_in: title match ---

    #[test]
    fn find_by_title_exact() {
        let page = find_in(TEST_PAGES, "What is Almanac?").unwrap();
        assert_eq!(page.slug, "what-is-almanac");
    }

    #[test]
    fn find_by_title_case_insensitive() {
        let page = find_in(TEST_PAGES, "cli reference").unwrap();
        assert_eq!(page.slug, "cli-reference");
    }

    // --- find_in: unique prefix ---

    #[test]
    fn find_by_unique_prefix() {
        let page = find_in(TEST_PAGES, "what").unwrap();
        assert_eq!(page.slug, "what-is-almanac");
    }

    #[test]
    fn find_ambiguous_prefix_returns_none() {
        // "cli" matches both "cli-reference" and "cli-advanced"
        assert!(find_in(TEST_PAGES, "cli").is_none());
    }

    #[test]
    fn find_nonexistent_returns_none() {
        assert!(find_in(TEST_PAGES, "nonexistent").is_none());
    }

    // --- format_list ---

    #[test]
    fn format_list_shows_slugs() {
        let output = format_list(TEST_PAGES);
        assert!(output.contains("what-is-almanac"));
        assert!(output.contains("cli-reference"));
        assert!(output.contains("cli-advanced"));
        // Should NOT show titles as the primary identifier
        assert!(!output.contains("What is Almanac?"));
    }

    #[test]
    fn format_list_shows_descriptions() {
        let output = format_list(TEST_PAGES);
        assert!(output.contains("Agent skill aggregation"));
        assert!(output.contains("Complete command reference"));
    }

    // --- find_matching ---

    #[test]
    fn search_matches_title() {
        let results = find_matching(TEST_PAGES, "almanac");
        assert_eq!(results.len(), 2); // title match + description match on cli-reference
    }

    #[test]
    fn search_matches_description() {
        let results = find_matching(TEST_PAGES, "aggregation");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].slug, "what-is-almanac");
    }

    #[test]
    fn search_matches_content() {
        let results = find_matching(TEST_PAGES, "power users");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].slug, "cli-advanced");
    }

    #[test]
    fn search_case_insensitive() {
        let results = find_matching(TEST_PAGES, "ADVANCED");
        assert!(results.iter().any(|p| p.slug == "cli-advanced"));
    }

    #[test]
    fn search_matches_slug() {
        let results = find_matching(TEST_PAGES, "cli-ref");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].slug, "cli-reference");
    }

    #[test]
    fn search_no_matches() {
        let results = find_matching(TEST_PAGES, "xyzzy");
        assert!(results.is_empty());
    }
}
