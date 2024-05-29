use crate::site_config::SiteConfig;
use std::path::PathBuf;

use crate::page_v2::PageV2;

impl PageV2 {
    //

    pub fn mock_1_with_ast() -> PageV2 {
        let content = r#"
-- title 

Mock File 1 With AST

-- metadata
-- id: alfa1234
-- created: 2024-05-20T10:11:12
"#
        .trim_start()
        .to_string();

        let mut p = PageV2 {
            ast: vec![],
            config: SiteConfig::mock1(),
            output: None,
            source_path: Some(PathBuf::from("/mock/root/content/some-file.neo")),
            source_content: Some(content),
        };
        let config = SiteConfig::mock1();
        p.generate_ast(&config);
        p
    }

    pub fn mock_2_home_page() -> PageV2 {
        let content = r#"
-- title 

Home Page Mock Up

-- metadata
-- id: bravo123 
-- title: Title From Metadata
-- type: home-page
-- path: /
"#
        .trim_start()
        .to_string();

        let mut p = PageV2 {
            ast: vec![],
            config: SiteConfig::mock1(),
            output: None,
            source_path: Some(PathBuf::from("/mock/root/content/home-page.neo")),
            source_content: Some(content),
        };
        let config = SiteConfig::mock1();
        p.generate_ast(&config);
        p
    }

    pub fn mock_3_bookmark_section() -> PageV2 {
        let content = r#"
-- bookmark
-- title: Title From Bookmark Attribute
-- url: https://www.example.com

Some text for the bookmark

-- metadata
-- id: charlie1
-- create: 2022-01-01
-- updated: 2022-02-02
-- status: draft
"#
        .trim_start()
        .to_string();
        let mut p = PageV2 {
            ast: vec![],
            config: SiteConfig::mock1(),
            output: None,
            source_path: Some(PathBuf::from("/mock/root/content/sub-dir/bookmark.neo")),
            source_content: Some(content),
        };
        let config = SiteConfig::mock1();
        p.generate_ast(&config);
        p
    }

    pub fn mock_4_title_from_text() -> PageV2 {
        let content = r#"
-- short

This is to test the title that comes in from the
first few words of the first section if no other
title is found

-- metadata
-- id: delta123
-- created: 2024-05-22
-- type: short
"#
        .trim_start()
        .to_string();
        let mut p = PageV2 {
            ast: vec![],
            config: SiteConfig::mock1(),
            output: None,
            source_path: Some(PathBuf::from("/mock/root/content/title-from-body-text.neo")),
            source_content: Some(content),
        };
        let config = SiteConfig::mock1();
        p.generate_ast(&config);
        p
    }

    pub fn mock_5_no_title() -> PageV2 {
        let content = r#"
-- metadata
-- id: echo1234
-- status: draft
-- updated: 2023-01-01
-- type: short
"#
        .trim_start()
        .to_string();
        let mut p = PageV2 {
            ast: vec![],
            config: SiteConfig::mock1(),
            output: None,
            source_path: Some(PathBuf::from("/mock/root/content/no-title.neo")),
            source_content: Some(content),
        };
        let config = SiteConfig::mock1();
        p.generate_ast(&config);
        p
    }

    pub fn mock_6_url_title_parsing() -> PageV2 {
        let content = r#"
-- metadata
-- id: foxtrot1 
-- created: 2023-01-02
-- title: - Another ' URL 42 ~ title -
"#
        .trim_start()
        .to_string();
        let mut p = PageV2 {
            ast: vec![],
            config: SiteConfig::mock1(),
            output: None,
            source_path: Some(PathBuf::from("/mock/root/content/foxtrot1.neo")),
            source_content: Some(content),
        };
        let config = SiteConfig::mock1();
        p.generate_ast(&config);
        p
    }

    pub fn mock_7_golf1234() -> PageV2 {
        let content = r#"
-- title 

TODO Example 1

-- todo

[] alfa

[] bravo

-- ref 
-- title: Golf 1234 Ref Alfa
-- url: https://golf1234-alfa.example.com/

-- ref 
-- title: Golf 1234 Ref Bravo
-- url: https://golf1234-bravo.example.com/

-- metadata
-- id: golf1234 
-- created: 2022-01-02
-- updated: 2023-01-02
"#
        .trim_start()
        .to_string();
        let mut p = PageV2 {
            ast: vec![],
            config: SiteConfig::mock1(),
            output: None,
            source_path: Some(PathBuf::from(
                "/mock/root/content/sub-dir1/sub-dir2/golf1234.neo",
            )),
            source_content: Some(content),
        };
        let config = SiteConfig::mock1();
        p.generate_ast(&config);
        p
    }

    pub fn mock_8_hotel123() -> PageV2 {
        let content = r#"
-- title 

Some Page Example

-- ref 
-- title: Hotel 123 Ref Alfa
-- url: https://hotel123-alfa.example.com/

-- ref 
-- title: Hotel 123 Ref Bravo
-- url: https://hotel123-bravo.example.com/

-- metadata
-- id: hotel123 
-- created: 2023-01-03
"#
        .trim_start()
        .to_string();
        let mut p = PageV2 {
            ast: vec![],
            config: SiteConfig::mock1(),
            output: None,
            source_path: Some(PathBuf::from(
                "/mock/root/content/sub-dir1/sub-dir2/golf1234.neo",
            )),
            source_content: Some(content),
        };
        let config = SiteConfig::mock1();
        p.generate_ast(&config);
        p
    }

    //
}
