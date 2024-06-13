use neopoligengine::span::*;
use rstest::rstest;

#[rstest]
#[case(1, "``alfa``")]
#[case(2, "``alfa-bravo``")]
#[case(3, "^^alfa^^")]
#[case(4, "^^alfa-bravo^^")]

fn generated_shorthand_base_cases(#[case] number: usize, #[case] source: &str) {
    assert!(base_span_for_all_text(source).is_ok());
}
    