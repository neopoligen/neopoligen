// // THIS FILE IS DEPRECATED 
// // TODO: Delete this when 
// // src/section/list_section_item is done




// use crate::config::Config;
// // use crate::list;
// // use crate::section::Section;
// use nom::IResult;
// use nom::character::complete::multispace0;
// use crate::child::Child;
// // use crate::child::child;
// // use crate::list::list;
// // use nom::bytes::complete::is_not;
// use nom::bytes::complete::tag;
// use nom::character::complete::space1;
// // use nom::sequence::pair;
// // use nom::bytes::complete::tag;
// // use nom::multi::many0;
// // use crate::span::Span;
// use crate::block::block;
// use crate::section::section;
// use nom::branch::alt;
// use nom::combinator::not;
// use nom::sequence::tuple;


// // #[derive(Debug, PartialEq)]
// // pub enum ListItem {
// //     Section(Section),
// //     Block(Vec<Span>),
// // }


// pub fn list_item<'a>(source: &'a str, config: &'a Config) -> IResult<&'a str, Child> {
//     let (source, _) = tuple((tag("-"), not(tag("-")), space1))(source)?;
//     // dbg!(&source);
//     // let (source, _) = block(source, &config)?;

//     let (source, item) = alt((
//         // TODO: Set up for multi pragraph
//         |src| block(src, config),
//         |src| section(src, config),
//     ))(source)?;
//     // dbg!(&source);

  
//     // let (source, r#type) = preceded(tag_no_case("-- "), is_not(" \n\t"))(source)?;
//     // dbg!(&source);
//     // let (source, _) = tag("\n\n")(source)?;
//     // dbg!(&source);
//     // let (source, list_items) = many0(list_item)(source)?;
//     // dbg!(&source);


    
//     // let (source, item) = section(source, r#type, config)?;
//     // let (source, sec) = alt((
//     //     |src| section_basic_full(src, r#type, config),
//     //     |src| section_basic_full(src, r#type, config)
//     // ))(source)?;

//     let (source, _) = multispace0(source)?;
//     Ok((
//         source,
//         item
//     ))
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use pretty_assertions::assert_eq;
//     use crate::span::Span;

//     #[test]
//     // #[ignore]
//     fn list_item_integration() {
//         let source = "- juliette foxtrot\n\n- whiskey\n\n-- p";
//         let config = Config::mock_basic_config();
//         let left = Ok((
//             "- whiskey\n\n-- p",
//             Child::Block(vec![
//                 Span::Word {
//                     text: "juliette".to_string(),
//                     template: "spans/word.jinja".to_string()
//                 },
//                 Span::Space {
//                     text: " ".to_string(),
//                     template: "spans/space.jinja".to_string()
//                 },
//                 Span::Word {
//                     text: "foxtrot".to_string(),
//                     template: "spans/word.jinja".to_string()
//                 }
//             ]),
//         ));
//         let right = list_item(source, &config);
//         assert_eq!(left, right);
//     }
// }
