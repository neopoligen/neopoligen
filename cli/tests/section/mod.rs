use neopoligengine::section::basic::*;
use neopoligengine::section::raw::*;
use neopoligengine::section::Section;
use neopoligengine::site_config::SiteConfig;
use neopoligengine::span::Span;
use nom::Parser;
use nom_supreme::ParserExt;
use pretty_assertions::assert_eq;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[test]
fn basic_section_full_with_attrs() {
    let attrs = BTreeMap::new();
    let attr_list = vec![];
    let bounds = "full".to_string();
    let children = vec![Section::Block {
        bounds: "full".to_string(),
        spans: vec![Span::WordPart {
            text: "alfa".to_string(),
            r#type: "wordpart".to_string(),
        }],
        r#type: "basic-block".to_string(),
    }];
    let mut flags = BTreeSet::new();
    flags.insert("flag".to_string());
    let r#type = "title".to_string();
    let config = SiteConfig::mock1();
    let source = "-- title\n-- flag\n-- key: value1\n\nalfa\n\n-- div...";
    let left = (
        "-- div...",
        Section::Basic {
            attrs,
            attr_list,
            bounds,
            children,
            flags,
            r#type,
        },
    );
    let right = basic_section_full(source, &config.sections, &config.spans).unwrap();
    assert_eq!(left, right);
}

#[test]
fn results_section_full() {
    let attrs = BTreeMap::new();
    let config = SiteConfig::mock1();
    let flags = vec![];
    let source = "-- results\n\nalfa bravo charlie";
    let left = Section::Raw {
        attrs,
        bounds: "full".to_string(),
        children: vec![],
        flags,
        text: Some("alfa bravo charlie".to_string()),
        r#type: "results".to_string(),
    };
    let right = raw_section_full(source, &config.sections, &config.spans)
        .unwrap()
        .1;
    assert_eq!(left, right);
}

#[test]
fn results_section_start_end() {
    let attrs = BTreeMap::new();
    let config = SiteConfig::mock1();
    let flags = vec![];
    let source = "-- results/\n\nalfa bravo charlie\n\n-- /results";
    let left = Section::Raw {
        attrs,
        bounds: "start".to_string(),
        children: vec![Section::Basic {
            attrs: BTreeMap::new(),
            bounds: "end".to_string(),
            children: vec![],
            flags: vec![],
            r#type: "results".to_string(),
        }],
        flags,
        text: Some("alfa bravo charlie".to_string()),
        r#type: "results".to_string(),
    };
    let right = raw_section_start(source, &config.sections, &config.spans)
        .unwrap()
        .1;
    assert_eq!(left, right);
}

#[test]
#[ignore]
fn section_with_em() {
    let config = SiteConfig::mock1();
    let source =
        "-- warning\n\nThis is still a draft\n\nof <<em|1s>> and 0s.\n\nThis is still a draft\n\n";
    let left = Section::Basic {
        attrs: BTreeMap::new(),
        flags: vec![],
        bounds: "full".to_string(),
        children: vec![Section::Block {
            bounds: "full".to_string(),
            spans: vec![
                Span::WordPart {
                    text: "of".to_string(),
                    r#type: "wordpart".to_string(),
                },
                Span::Space {
                    text: " ".to_string(),
                    r#type: "space".to_string(),
                },
                Span::KnownSpan {
                    attrs: BTreeMap::new(),
                    flags: vec![],
                    spans: vec![Span::WordPart {
                        text: "1s".to_string(),
                        r#type: "wordpart".to_string(),
                    }],
                    r#type: "em".to_string(),
                },
                Span::Space {
                    text: " ".to_string(),
                    r#type: "space".to_string(),
                },
                Span::WordPart {
                    text: "and".to_string(),
                    r#type: "wordpart".to_string(),
                },
                Span::Space {
                    text: " ".to_string(),
                    r#type: "space".to_string(),
                },
                Span::WordPart {
                    text: "0s.".to_string(),
                    r#type: "wordpart".to_string(),
                },
            ],
            r#type: "basic-block".to_string(),
        }],
        r#type: "warning".to_string(),
    };
    let right = (|src| basic_section_full(src, &config.sections, &config.spans))
        .context("")
        .parse(source)
        .unwrap()
        .1;
    assert_eq!(left, right);
}
