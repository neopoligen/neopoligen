use neopoligengine::span::*;
use rstest::rstest;

#[rstest]
#[case(1, "``alfa bravo``", 0, 0)]
#[case(2, "``alfa-bravo``", 0, 0)]
#[case(3, "``alfa_bravo``", 0, 0)]
#[case(4, "``alfa`bravo``", 0, 0)]
#[case(5, "``alfa:bravo``", 0, 0)]
#[case(6, "^^alfa bravo^^", 0, 0)]
#[case(7, "^^alfa-bravo^^", 0, 0)]
#[case(8, "^^alfa_bravo^^", 0, 0)]
#[case(9, "^^alfa`bravo^^", 0, 0)]
#[case(10, "^^alfa:bravo^^", 0, 0)]

fn generated_shorthand_base_cases(
    #[case] _number: usize, 
    #[case] source: &str, 
    #[case] flag_count: usize, 
    #[case] kv_count: usize
    ) {
    let span = shorthand(source).unwrap().1;
    
}
    