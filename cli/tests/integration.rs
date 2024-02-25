mod integration {
    use minijinja::{context, Environment};
    use neopoligen_cli::config::Config;
    use neopoligen_cli::page::Page;
    use neopoligen_cli::site::Site;
    use pretty_assertions::assert_eq;
    use std::path::PathBuf;

    fn load_templates(env: &mut Environment) {
        env.add_template_owned(
            "splitter.jinja",
            r#"{%- import "includes/macros.jinja" as macros -%}
{%- include "global_vars" -%}
{%- for page_id in site.page_ids() -%}
{# {{ macros.log(page_id, "") }} #}
{{ site.output_path_for_page(page_id) }}
--- PAGE_DATA_SPLIT ---
{% include site.template_for_page(page_id) %}
--- PAGE_SEPERATOR ---
{% endfor -%}"#
                .to_string(),
        )
        .unwrap();
        env.add_template_owned(
            "pages/post/published.jinja",
            "This is the published post page".to_string(),
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
        let page = Page::new(
            PathBuf::from("asdf"),
            "asdf".to_string(),
            &Config::site1_config(),
        );

        // site.pages.in

        // let mut env = Environment::new();
        // load_templates(&mut env);
        // let skeleton = env.get_template("pages/post/published.jinja").unwrap();
        // let left = "This is the published post page".to_string();
        // let right = skeleton.render(context!(name => "World")).unwrap();
        // assert_eq!(left, right);
    }
}
