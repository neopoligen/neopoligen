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
-- data: 2024-05-20T10:11:12
"#
        .trim_start()
        .to_string();

        let mut p = PageV2 {
            ast: vec![],
            cached_hash: None,
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
-- data: 2024-05-20T10:11:12
-- path: /
"#
        .trim_start()
        .to_string();

        let mut p = PageV2 {
            ast: vec![],
            cached_hash: None,
            config: SiteConfig::mock1(),
            output: None,
            source_path: Some(PathBuf::from("/mock/root/content/home-page.neo")),
            source_content: Some(content),
        };
        let config = SiteConfig::mock1();
        p.generate_ast(&config);
        p
    }

    //
}