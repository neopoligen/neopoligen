mod integration {
    use minijinja::{context, Environment};
    use neopoligen_cli::site::Site;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn basic_test() {
        let site = Site::site1();
        let mut env = Environment::new();
        env.add_template_owned(
            "pages/post/published.jinja",
            "This is the published post page".to_string(),
        )
        .unwrap();
        let skeleton = env.get_template("pages/post/published.jinja").unwrap();
        let left = "This is the published post page".to_string();
        let right = skeleton.render(context!(name => "World")).unwrap();
        assert_eq!(left, right);
    }
}
