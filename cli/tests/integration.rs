mod integration {
    use minijinja::{context, Environment};
    use neopoligen_cli::site::Site;
    use pretty_assertions::assert_eq;

    fn load_templates(env: &mut Environment) {
        env.add_template_owned(
            "pages/post/published.jinja",
            "This is the published post page".to_string(),
        )
        .unwrap();
    }

    #[test]
    fn basic_test() {
        let site = Site::site1();
        let mut env = Environment::new();
        load_templates(&mut env);
        let skeleton = env.get_template("pages/post/published.jinja").unwrap();
        let left = "This is the published post page".to_string();
        let right = skeleton.render(context!(name => "World")).unwrap();
        assert_eq!(left, right);
    }
}
