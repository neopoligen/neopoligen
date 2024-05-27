use neopoligengine::image::Image;
use pretty_assertions::assert_eq;
// use std::collections::BTreeSet;

#[test]
fn extension_is_lowercase() {
    let image = Image::mock_1();
    let left = "jpg".to_string();
    let right = image.extension().unwrap();
    assert_eq!(left, right);
}

#[test]
fn key_basic() {
    let image = Image::mock_1();
    let left = "image-name".to_string();
    let right = image.key().unwrap();
    assert_eq!(left, right);
}

#[test]
fn set_version_dimensions() {
    let mut image = Image::mock_1();
    image.width = Some(400);
    image.height = Some(300);
    let _ = image.set_dimensions(vec![100, 300]);
    let left = image.versions[1].1;
    let right = 225;
    assert_eq!(left, right);
}

#[test]
fn stop_at_max_width() {
    let mut image = Image::mock_1();
    image.width = Some(400);
    image.height = Some(300);
    let _ = image.set_dimensions(vec![100, 300, 500]);
    let left = image.versions[2].0;
    let right = 400;
    assert_eq!(left, right);
}

// #[test]
// fn output_sizes_basic() {
//     let image = Image::mock_1();
//     let mut left = BTreeSet::new();
//     left.insert(100_u32);
//     left.insert(300_u32);
//     left.insert(400_u32);
//     left.insert(600_u32);
//     let right = image.output_widths();
//     assert_eq!(left, right);
// }
