use neopoligengine::image::Image;
use pretty_assertions::assert_eq;

#[test]
fn key_basic() {
    let image = Image::mock_1();
    let left = "image-name".to_string();
    let right = image.key().unwrap();
    assert_eq!(left, right);
}
