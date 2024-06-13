use neopoligengine::span::*;
use neopoligengine::span_attr::*;
use rstest::rstest;

#[rstest]
#[case(1, r#"``alfa bravo``"#, 0, 0)]
#[case(2, r#"``alfa`bravo``"#, 0, 0)]
#[case(3, r#"``alfa-bravo``"#, 0, 0)]
#[case(4, r#"``alfa_bravo``"#, 0, 0)]
#[case(5, r#"``alfa:bravo``"#, 0, 0)]
#[case(6, r#"``alfa<bravo``"#, 0, 0)]
#[case(7, r#"``alfa>bravo``"#, 0, 0)]
#[case(8, r#"``alfa^bravo``"#, 0, 0)]
#[case(9, r#"``alfa\|bravo``"#, 0, 0)]
#[case(10, r#"``alfa\``bravo``"#, 0, 0)]
#[case(11, r#"``alfa\-bravo``"#, 0, 0)]
#[case(12, r#"``alfa\_bravo``"#, 0, 0)]
#[case(13, r#"``alfa\--bravo``"#, 0, 0)]
#[case(14, r#"``alfa\__bravo``"#, 0, 0)]
#[case(15, r#"``alfa\:bravo``"#, 0, 0)]
#[case(16, r#"``alfa\::bravo``"#, 0, 0)]
#[case(17, r#"``alfa\\bravo``"#, 0, 0)]
#[case(18, r#"``alfa bravo|charlie``"#, 1, 0)]
#[case(19, r#"``alfa bravo|charlie|delta``"#, 2, 0)]
#[case(20, r#"``alfa bravo|https://www.example.com/``"#, 1, 0)]
#[case(21, r#"^^alfa bravo^^"#, 0, 0)]
#[case(22, r#"^^alfa`bravo^^"#, 0, 0)]
#[case(23, r#"^^alfa-bravo^^"#, 0, 0)]
#[case(24, r#"^^alfa_bravo^^"#, 0, 0)]
#[case(25, r#"^^alfa:bravo^^"#, 0, 0)]
#[case(26, r#"^^alfa<bravo^^"#, 0, 0)]
#[case(27, r#"^^alfa>bravo^^"#, 0, 0)]
#[case(28, r#"^^alfa^bravo^^"#, 0, 0)]
#[case(29, r#"^^alfa\|bravo^^"#, 0, 0)]
#[case(30, r#"^^alfa\``bravo^^"#, 0, 0)]
#[case(31, r#"^^alfa\-bravo^^"#, 0, 0)]
#[case(32, r#"^^alfa\_bravo^^"#, 0, 0)]
#[case(33, r#"^^alfa\--bravo^^"#, 0, 0)]
#[case(34, r#"^^alfa\__bravo^^"#, 0, 0)]
#[case(35, r#"^^alfa\:bravo^^"#, 0, 0)]
#[case(36, r#"^^alfa\::bravo^^"#, 0, 0)]
#[case(37, r#"^^alfa\\bravo^^"#, 0, 0)]
#[case(38, r#"^^alfa bravo|charlie^^"#, 1, 0)]
#[case(39, r#"^^alfa bravo|charlie|delta^^"#, 2, 0)]
#[case(40, r#"^^alfa bravo|https://www.example.com/^^"#, 1, 0)]

fn generated_shorthand_base_cases(
    #[case] _number: usize, 
    #[case] source: &str, 
    #[case] flag_count: usize, 
    #[case] kv_count: usize
    ) {
    let span = shorthand(source).unwrap().1;
    let kv = span.attrs.iter().filter_map(|attr|{
        match &attr.kind {
            SpanAttrKind::KeyValue { .. } => Some(()),
            _ => None
        }
    }).collect::<Vec<()>>();
    let flags = span.attrs.iter().filter_map(|attr|{
        match &attr.kind {
            SpanAttrKind::Flag { .. } => Some(()),
            _ => None
        }
    }).collect::<Vec<()>>();
    assert_eq!(kv.len(), kv_count);
    assert_eq!(flags.len(), flag_count);
}
    