use crate::helpers::get_file_paths_for_extension::get_file_paths_for_extension;
use crate::site_builder::SiteBuilder;
//use minijinja::context;
use minijinja::Environment;
use minijinja::Syntax;
use std::fs;
// use tracing::{event, instrument, span, Level};
// use minijinja::path_loader;

impl SiteBuilder<'_> {
    //#[instrument]
    pub fn load_templates(&mut self) {
        // Load the main template directory first
        // self.env.set_loader(path_loader(&self.config.folders.templates_root));

        // TODO: Load json-plugin and text-plugin templates for real
        // event!(Level::INFO, "load json and todo templates");
        self.env
            .add_template_owned("sections/json-plugin/full/default.jinja", "todo")
            .unwrap();

        self.env
            .add_template_owned("sections/json-plugin/start/default.jinja", "todo")
            .unwrap();

        self.env
            .add_template_owned("sections/json-plugin/end/default.jinja", "todo")
            .unwrap();

        self.env
            .add_template_owned("sections/text-plugin/full/default.jinja", "todo")
            .unwrap();

        self.env
            .add_template_owned("sections/text-plugin/start/default.jinja", "todo")
            .unwrap();

        self.env
            .add_template_owned("sections/text-plugin/end/default.jinja", "todo")
            .unwrap();

        let file_system_template_dirs = [
            "helpers",
            "includes",
            "page-types",
            "sections",
            "spans",
            "wrappers",
        ];

        // event!(Level::INFO, "get file_system_template_dirs");
        file_system_template_dirs.iter().for_each(|dir| {
            let mut dir_path = self.config.folders.theme_root.clone();
            dir_path.push(dir);
            get_file_paths_for_extension(&dir_path, "jinja")
                .iter()
                .for_each(|path| {
                    let the_content = fs::read_to_string(path).unwrap();
                    let name = path
                        .strip_prefix(self.config.folders.theme_root.clone())
                        .unwrap();
                    // .clone();
                    // dbg!(&name);
                    self.env
                        .add_template_owned(name.display().to_string(), the_content)
                        .unwrap();
                });
        });

        // event!(Level::INFO, "make: skeleton_env");
        let mut skeleton_env = Environment::new();
        // event!(Level::INFO, "made: skeleton_env");
        skeleton_env
            .set_syntax(Syntax {
                block_start: "NA_BLOCK_START".into(),
                block_end: "NA_BLOCK_END".into(),
                variable_start: "{$".into(),
                variable_end: "$}".into(),
                comment_start: "NA_COMMENT_START".into(),
                comment_end: "NA_COMMENT_END".into(),
            })
            .unwrap();
        // event!(Level::INFO, "skeleton_env.set_syntax()");

        // I Goofed on this structure, it should have been a BTreeMap
        // or something. Also, there's probably a way to eliminate
        // the duplication, but I don't have the energy to look right now

        // let my_span = span!(Level::INFO, "standard_templates");
        // my_span.in_scope(|| {
        //     // dbg!(&skeleton_env);
        //     bounds.iter().for_each(|bound| {
        //         let mut standard_input_path =
        //             self.config.folders.theme_default_sections_root.clone();
        //         standard_input_path.push("standard");
        //         standard_input_path.push(format!("{}.jinja", bound));
        //         let standard_skeleton = fs::read_to_string(&standard_input_path).unwrap();
        //         skeleton_env
        //             .add_template_owned(format!("standard_{}", &bound), standard_skeleton)
        //             .unwrap();
        //         self.config
        //             .section_categories
        //             .standard
        //             .iter()
        //             .for_each(|name| {
        //                 // skeleton_env.clear_templates();
        //                 // let mut input_path = self.config.folders.theme_default_sections_root.clone();
        //                 // input_path.push("standard");
        //                 // input_path.push(format!("{}.jinja", bound));
        //                 let prod_template_name =
        //                     format!("sections/{}/{}/default.jinja", &name, &bound);
        //                 // event!(Level::INFO, "Reading File");
        //                 // let skeleton = fs::read_to_string(&input_path).unwrap();
        //                 // // event!(Level::INFO, "Done Reading File");
        //                 // skeleton_env
        //                 //     .add_template_owned("section_template", skeleton)
        //                 //     .unwrap();
        //                 // event!(Level::INFO, "skeleton_env.add_template_owned()");
        //                 let tmpl = skeleton_env
        //                     .get_template(format!("standard_{}", &bound).as_str())
        //                     .unwrap();
        //                 // event!(Level::INFO, "skeleton_env.get_template()");
        //                 let output = tmpl.render(context!(SECTION_KEY => &name)).unwrap();
        //                 // event!(Level::INFO, "tmpl.render()");
        //                 self.env
        //                     .add_template_owned(prod_template_name, output)
        //                     .unwrap();
        //                 // event!(Level::INFO, "self.env.add_template_owned()");
        //             })
        //     });
        // });
        // // dbg!(&skeleton_env);
    }
}
