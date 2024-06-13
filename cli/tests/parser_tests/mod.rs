use neopoligengine::span::*;
use neopoligengine::span_attr::*;
use rstest::rstest;

#[rstest]
#[case(1, "``alfa bravo``", 0, 0)]
#[case(2, "``alfa`bravo``", 0, 0)]
#[case(3, "``alfa-bravo``", 0, 0)]
#[case(4, "``alfa_bravo``", 0, 0)]
#[case(5, "``alfa:bravo``", 0, 0)]
#[case(6, "``alfa<bravo``", 0, 0)]
#[case(7, "``alfa>bravo``", 0, 0)]
#[case(8, "``alfa^bravo``", 0, 0)]
#[case(9, "``alfa\\|bravo``", 0, 0)]
#[case(10, "``alfa\\``bravo``", 0, 0)]
#[case(11, "``alfa\\-bravo``", 0, 0)]
#[case(12, "``alfa\\_bravo``", 0, 0)]
#[case(13, "``alfa\\--bravo``", 0, 0)]
#[case(14, "``alfa\\__bravo``", 0, 0)]
#[case(15, "``alfa\\:bravo``", 0, 0)]
#[case(16, "``alfa\\::bravo``", 0, 0)]
#[case(17, "``alfa\\\\bravo``", 0, 0)]
#[case(18, "^^alfa bravo^^", 0, 0)]
#[case(19, "^^alfa`bravo^^", 0, 0)]
#[case(20, "^^alfa-bravo^^", 0, 0)]
#[case(21, "^^alfa_bravo^^", 0, 0)]
#[case(22, "^^alfa:bravo^^", 0, 0)]
#[case(23, "^^alfa<bravo^^", 0, 0)]
#[case(24, "^^alfa>bravo^^", 0, 0)]
#[case(25, "^^alfa^bravo^^", 0, 0)]
#[case(26, "^^alfa\\|bravo^^", 0, 0)]
#[case(27, "^^alfa\\``bravo^^", 0, 0)]
#[case(28, "^^alfa\\-bravo^^", 0, 0)]
#[case(29, "^^alfa\\_bravo^^", 0, 0)]
#[case(30, "^^alfa\\--bravo^^", 0, 0)]
#[case(31, "^^alfa\\__bravo^^", 0, 0)]
#[case(32, "^^alfa\\:bravo^^", 0, 0)]
#[case(33, "^^alfa\\::bravo^^", 0, 0)]
#[case(34, "^^alfa\\\\bravo^^", 0, 0)]

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
    