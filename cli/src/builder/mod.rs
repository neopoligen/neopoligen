pub mod new;

use crate::config::Config;
use crate::file_set::FileSet;
use crate::site::Site;
use minijinja::context;
use minijinja::Environment;
use minijinja::Value;
use std::path::PathBuf;

pub struct Builder {
    file_set: FileSet,
    config: Config,
}

impl Builder {
    pub fn file_to_output(&self) -> Vec<(PathBuf, String)> {
        let mut env = Environment::new();
        let site = Site::new(&self.file_set, &self.config);

        self.file_set
            .templates
            .iter()
            .for_each(|t| env.add_template_owned(t.0, t.1).unwrap());

        env.add_template_owned(
            "splitter.jinja".to_string(),
            r#"{#- import "includes/macros.jinja" as macros -#}
{#- include "global_vars" -#}
{%- for page_id in site.page_ids() -%}
{{ site.page_output_path(page_id) }}
--- PAGE_DATA_SPLIT ---
{# include site.page_template(page_id) #}
--- PAGE_SEPARATOR ---
{% endfor -%}"#
                .to_string(),
        )
        .unwrap();

        match env.get_template("splitter.jinja") {
            Ok(splitter) => {
                match splitter.render(context!(
                     site => Value::from_object(site),
                )) {
                    Ok(combined_pages) => {
                        dbg!(combined_pages);
                        vec![]
                    }
                    Err(e) => {
                        println!("{}", e);
                        vec![]
                    }
                }
            }
            Err(e) => {
                println!("{}", e);
                vec![]
            }
        }
    }
}
