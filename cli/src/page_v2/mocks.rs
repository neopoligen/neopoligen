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
-- id: abcd1234
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
-- created: 2020-01-01T10:11:12
-- updated: 2022-10-10
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
-- id: charlie3737 
-- status: draft
"#
        .trim_start()
        .to_string();
        let mut p = PageV2 {
            ast: vec![],
            config: SiteConfig::mock1(),
            output: None,
            source_path: Some(PathBuf::from("/mock/root/content/sub-dir/bookmakr.neo")),
            source_content: Some(content),
        };
        let config = SiteConfig::mock1();
        p.generate_ast(&config);
        p
    }

    pub fn mock_4_title_from_text() -> PageV2 {
        let content = r#"
-- div 

This is to test the title that comes in from the
first few words of the first section if no other
title is found

-- metadata
-- id: delta7262 
-- created: 2024-05-22
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
-- id: echo8171 
-- updated: 2024-05-22T10:11:12
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
-- id: abcd1234
-- created: 2024-05-20
-- title: - Another ' URL 42 ~ title -
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

    //
}
