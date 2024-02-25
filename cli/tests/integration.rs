mod integration {
    use minijinja::{context, Environment, Value};
    // use neopoligen_cli::config::Config;
    use neopoligen_cli::page::Page;
    use neopoligen_cli::site::Site;
    // use pretty_assertions::assert_eq;
    // use std::path::PathBuf;

    fn load_global_vars(env: &mut Environment) {
        env.add_template_owned("global_vars", "".to_string())
            .unwrap();
    }

    fn load_templates(env: &mut Environment) {
        env.add_template_owned("includes/macros.jinja", "".to_string())
            .unwrap();
    }

    fn load_splitter(env: &mut Environment) {
        env.add_template_owned(
            "splitter.jinja",
            r#"{%- import "includes/macros.jinja" as macros -%}
{%- include "global_vars" -%}
{%- for page_id in site.page_ids() -%}PADSASDASD
{# {{ macros.log(page_id, "") }} #}
{# site.output_path_for_page(page_id) #}
--- PAGE_DATA_SPLIT ---
{# include site.template_for_page(page_id) #}
--- PAGE_SEPERATOR ---
{% endfor -%}"#
                .to_string(),
        )
        .unwrap();
    }

    #[test]
    fn single_page_test() {
        let unsplit_content = r#"/en/_index/?integration-site-home-page
--- PAGE_DATA_SPLIT ---
This is the page output
--- PAGE_SEPERATOR ---
"#
        .to_string();
        let mut site = Site::new();
        let page = Page::s1_index();
        site.pages.insert(page.id().unwrap(), page);
        let mut env = Environment::new();
        load_splitter(&mut env);
        load_global_vars(&mut env);
        load_templates(&mut env);
        let skeleton = env.get_template("splitter.jinja").unwrap();

        // let left = "This is the published post page".to_string();
        let right = skeleton
            .render(context!(site => 
        Value::from_object(site)))
            .unwrap();
        dbg!(right);
        // assert_eq!(left, right);
    }
}
