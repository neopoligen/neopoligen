mod integration_solo {
    use minijinja::{context, Environment, Value};
    // use neopoligen_cli::config::Config;
    use neopoligen_cli::config::Config;
    use neopoligen_cli::page::Page;
    use neopoligen_cli::site::Site;
    use pretty_assertions::assert_eq;
    // use std::path::PathBuf;
    use std::collections::BTreeMap;

    fn load_global_vars(env: &mut Environment) {
        env.add_template_owned("global_vars", "".to_string())
            .unwrap();
    }

    fn load_templates(env: &mut Environment) {
        env.add_template_owned("includes/macros.jinja", "".to_string())
            .unwrap();
        env.add_template_owned(
            "pages/post/published.jinja",
            "This is the page output".to_string(),
        )
        .unwrap();
    }

    fn load_splitter(env: &mut Environment) {
        env.add_template_owned(
            "splitter.jinja",
            r#"{%- import "includes/macros.jinja" as macros -%}
{%- include "global_vars" -%}
{%- for page_id in site.page_ids() -%}
{{ site.page_output_path(page_id) }}
--- PAGE_DATA_SPLIT ---
{% include site.page_template(page_id) %}
--- PAGE_SEPARATOR ---
{% endfor -%}"#
                .to_string(),
        )
        .unwrap();
    }

    fn create_outputs(env: &Environment, site: Site) -> Vec<(String, String)> {
        let skeleton = env.get_template("splitter.jinja").unwrap();
        match skeleton.render(context!(site => Value::from_object(site))) {
            Ok(all_pages) => all_pages
                .split("--- PAGE_SEPARATOR ---")
                .filter_map(|page| {
                    let page_parts: Vec<_> = page.split("--- PAGE_DATA_SPLIT ---").collect();
                    if page_parts.len() == 2 {
                        Some((
                            page_parts[0].trim().to_string(),
                            page_parts[1].trim().to_string(),
                        ))
                    } else {
                        None
                    }
                })
                .collect(),
            Err(e) => {
                println!("{}", e);
                vec![]
            }
        }
    }

    #[test]
    // #[ignore]
    fn single_page_test() {
        let config = Config::site1_config();
        let mut site = Site::new(config);
        let page = Page::s1_index();
        site.pages.insert(page.id().unwrap(), page);
        let mut env = Environment::new();
        load_splitter(&mut env);
        load_global_vars(&mut env);
        load_templates(&mut env);
        // let skeleton = env.get_template("splitter.jinja").unwrap();
        let outputs = create_outputs(&env, site);
        dbg!("------------------------------");
        dbg!(outputs);

        // let left = r#"leading-dir/Neopoligen/integration-site/docs/en/id_index/index.html
        // --- PAGE_DATA_SPLIT ---
        // This is the page output
        // --- PAGE_SEPERATOR ---
        // "#
        // .to_string();
        // let right = skeleton
        //     .render(context!(site =>
        // Value::from_object(site)))
        //     .unwrap();
        // assert_eq!(left, right);
    }
}
