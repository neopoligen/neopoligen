use neopoligengine::{page_v2::PageV2, site_config::SiteConfig};
use pretty_assertions::assert_eq;

#[test]
fn id_basic() {
    let p = PageV2::mock_1_with_ast();
    let left = "abcd1234".to_string();
    let right = p.id().unwrap();
    assert_eq!(left, right)
}
