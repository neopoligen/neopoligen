pub mod new;

use crate::config::Config;
use crate::file_set::FileSet;
use crate::neo_config::NeoEnv;
use crate::site::Site;
use crate::template_error::TemplateError;
use dirs::config_local_dir;
use fs_extra::dir::copy;
use minijinja::context;
use minijinja::Environment;
use minijinja::Syntax;
use minijinja::Value;
use std::collections::BTreeMap;
use std::fs;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::time::Instant;
use tracing::{event, instrument, Level};

pub struct Builder {
    file_set: FileSet,
    config: Config,
    neo_env: NeoEnv,
    pub template_errors: Vec<TemplateError>,
    pub outputs: BTreeMap<PathBuf, String>,
}

impl Builder {
    pub fn copy_asset_folders(&self) {
        let now = Instant::now();
        let asset_folders = vec!["files", "images", "mp3s", "scripts"];
        let mut options = fs_extra::dir::CopyOptions::new();
        options.overwrite = true;
        // options.content_only = true;
        asset_folders.iter().for_each(|folder| {
            event!(Level::INFO, "Copying: {}", &folder);
            let mut source_folder = self.config.folders.project_root.clone();
            source_folder.push(folder);
            //let mut dest_folder = self.config.folders.build_root.clone();
            //dest_folder.push(folder);
            //let _ = verify_dir(&PathBuf::from(&dest_folder));
            let _ = verify_dir(&self.config.folders.build_root);
            match copy(source_folder, &self.config.folders.build_root, &options) {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            }
        });
        event!(Level::DEBUG, "||{:?}||", now.elapsed());
    }

    pub fn copy_theme_assets(&self) {
        let mut options = fs_extra::dir::CopyOptions::new();
        options.overwrite = true;
        options.content_only = true;
        let in_dir = self
            .config
            .folders
            .theme_assets_input_root
            .display()
            .to_string();
        let site_build_root_dir = self
            .config
            .folders
            .theme_assets_build_root
            .display()
            .to_string();
        match copy(in_dir, site_build_root_dir, &options) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
    }

    #[instrument(skip(self))]
    pub fn generate_files(&mut self) {
        let mut env = Environment::new();
        env.set_syntax(Syntax {
            block_start: "[!".into(),
            block_end: "!]".into(),
            variable_start: "[@".into(),
            variable_end: "@]".into(),
            comment_start: "[#".into(),
            comment_end: "#]".into(),
        })
        .unwrap();
        let site = Site::new(&self.file_set, &self.config);
        let site_obj = Value::from_serializable(&site.clone());
        self.file_set
            .templates
            .iter()
            .for_each(|t| env.add_template_owned(t.0, t.1).unwrap());
        site.pages.iter().for_each(|p| {
            let page = p.1;
            dbg!("----------------------");
            dbg!(format!(
                "{:?} - {:?} - {:?}",
                &page.id.clone(),
                &page.base_template.clone(),
                &page.status.clone()
            ));
            let template_searches = vec![
                format!(
                    "pages/{}/{}.neojinja",
                    &page.base_template.clone().unwrap(),
                    &page.status.clone().unwrap(),
                ),
                format!(
                    "pages/{}/published.neojinja",
                    &page.base_template.clone().unwrap()
                ),
                format!("pages/post/{}.neojinja", &page.status.clone().unwrap()),
                format!("pages/post/published.neojinja"),
            ];
            if let Some(template_name) =
                template_searches
                    .iter()
                    .find_map(|t| match &site.templates.get(t) {
                        Some(_) => Some(t),
                        None => None,
                    })
            {
                if let Ok(tmpl) = env.get_template(template_name) {
                    match tmpl.render(context!(
                         site => site_obj,
                        page_id => page.id
                    )) {
                        Ok(output) => {
                            self.outputs.insert(
                                PathBuf::from(&page.output_file_path.clone().unwrap()),
                                output,
                            );
                            ()
                        }
                        Err(e) => event!(Level::ERROR, "Error: {}", e),
                    }
                } else {
                    event!(Level::ERROR, "Could not get template: {}", template_name);
                }
            }
        });
    }

    #[instrument(skip(self))]
    pub fn move_files_in_place(&self) {
        if self.config.folders.output_root.exists() {
            let _ = fs::remove_dir_all(&self.config.folders.output_root);
        }
        let _ = fs::rename(
            &self.config.folders.build_root,
            &self.config.folders.output_root,
        );
    }

    #[instrument(skip(self))]
    pub fn output_files(&self) {
        self.outputs.iter().for_each(|output| {
            if output
                .0
                .starts_with(self.config.folders.build_root.display().to_string())
            {
                let build_path = PathBuf::from(output.0);
                let parent_dir = build_path.parent().unwrap();
                let _ = create_dir_all(parent_dir);
                let _ = fs::write(build_path, output.1);
            } else {
                println!("ERROR: Tried to write outside of the output root");
            }
            event!(Level::INFO, "Writing: {}", output.0.display());
        });
    }
}

// TODO: if you ever have to mess with this, change it to use
// a Result return type
fn file_exists(path: &PathBuf) -> bool {
    match path.try_exists() {
        Ok(exists) => {
            if exists == true {
                true
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

fn verify_dir(dir: &PathBuf) -> std::io::Result<()> {
    if dir.exists() {
        Ok(())
    } else {
        fs::create_dir_all(dir)
    }
}
