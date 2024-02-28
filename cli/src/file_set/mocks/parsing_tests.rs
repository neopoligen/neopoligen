use crate::file_set::FileSet;
use std::path::PathBuf;

impl FileSet {
    pub fn parsing_tests() -> FileSet {
        let mut fs = FileSet::new();
        fs.pages.insert(
            PathBuf::from("leading-dir/Neopoligen/parsing-tests/content/code-section-test.neo"),
            r#"-- title
-- subtitle: An app for making websites

Neopoligen

Neopoligen is a website builder.

It comes with lots of features built in. It's also 
very customizable. You can use it as-is or dig in 
and customize it completely.


-- aside

NOTE: Neopoligen is a work in progress. There are rough
edges, cryptic error messages, and bugs that have yet to 
be found. If you've got a little experience playing with 
early software you'll be fine. Otherwise, you'll probably 
want to come back in March to see how things are shaping up.


-- h2

How It Works

Neopoligen sites are made from text files that look
like this:

-- pre/
-- neo

-- title
-- subtitle: An app for making websites

Neopoligen

Neopoligen is a website builder.

It comes with lots of features built in. It's also 
very customizable. You can use it as-is or dig in 
and customize it completely. 

-- /pre

Those files get combined with templates to build a
static site that can be viewed locally and deployed
anywhere that offers static site hosting.




-- h2

Sections And Spans

Files are made up of "Sections" and "Spans". 
Sections are things like `-- title`` in the above
example. Spans looks like this `<<span|some text>>``

You can <<ilink: 2cgceffj|learn more about sections here>> 

You can <<ilink: 2ch7uk0v|learn more about spans here>> 



-- metadata
-- date: 2024-01-13 14:35:03
-- id: 2auhjdnh
-- path: /

                                 
"#
            .to_string(),
        );

        fs.templates.insert(
            "pages/post/published.jinja".to_string(),
            r#"This is a stub page"#.to_string(),
        );

        fs
    }
}
