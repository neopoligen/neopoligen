#!/Users/alan/.cargo/bin/cargo +nightly -Zscript
```cargo
[dependencies]
walkdir = "2"
```

use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

// This script makes generic versions of all the
// sections and then copies in customized ones. This
// is what should be used for making everything until
// the first launch

fn main() {
    let bounds = vec!["full", "start", "end"];
    let config_dir = PathBuf::from("lists");
    let files = get_files_in_a_single_dir(config_dir.clone());
    make_section_dirs(bounds.clone(), config_dir.clone(), files.clone());
    make_stubs(bounds.clone(), config_dir.clone(), files.clone());
    make_category_txt_files(bounds.clone(), config_dir.clone(), files.clone());
    copy_customized_files();
}

fn copy_customized_files() {
    let input_dir = PathBuf::from("customized");
    let output_dir = PathBuf::from("../../sections");

    get_files_in_dir_matching_extensions_recursively(
        &input_dir, 
        vec!["jinja"]
    ).iter().for_each(|in_path| {
        let mut out_path = output_dir.clone();
        out_path.push(in_path.strip_prefix(&input_dir).unwrap());
        let _ = make_parent_dir_for_file(&out_path);
        // dbg!(&in_path);
        // dbg!(&out_path);
        let _ = fs::copy(&in_path, &out_path);
    });

    get_files_in_dir_matching_extensions_recursively(
        &input_dir, 
        vec!["txt"]
    ).iter().for_each(|in_path| {
        let mut out_path = output_dir.clone();
        out_path.push(in_path.strip_prefix(&input_dir).unwrap());
        let _ = make_parent_dir_for_file(&out_path);
        // dbg!(&in_path);
        // dbg!(&out_path);
        let _ = fs::copy(&in_path, &out_path);
    });
}

fn make_section_dirs(bounds: Vec<&str>, _config_dir: PathBuf, files: Vec<PathBuf>) {
    files.iter().for_each(|f| {
        // let section_type = f.file_stem().unwrap();
        let file_data = fs::read_to_string(f).unwrap();
        let lines: Vec<&str> = file_data.lines().collect();
        lines.iter().for_each(|line| {
            bounds.iter().for_each(|bound| {
                let out_path =
                    PathBuf::from(format!("../../sections/{}/{}/default.jinja", line, bound));
                // dbg!(&out_path);
                if let Some(parent_dir) = out_path.parent() {
                    match fs::create_dir_all(parent_dir) {
                        Ok(_) => (),
                        Err(_) => (),
                    }
                };
            })
        })
    })
}

fn make_stubs(bounds: Vec<&str>, _config_dir: PathBuf, files: Vec<PathBuf>) {
    files.iter().for_each(|f| {
        let section_category = f.file_stem().unwrap();
        let file_data = fs::read_to_string(f).unwrap();
        let section_types: Vec<&str> = file_data.lines().collect();
        section_types.iter().for_each(|section_type| {
            bounds.iter().for_each(|bound| {
                let input_path = PathBuf::from(format!(
                    "default/{}/{}.jinja",
                    section_category.to_str().unwrap(),
                    bound
                ));
                let output_path = PathBuf::from(format!(
                    "../../sections/{}/{}/default.jinja",
                    section_type, bound
                ));
                let input_data = fs::read_to_string(&input_path).unwrap();
                // dbg!(&input_data);
                let updated_data = input_data.replace("SECTIONTYPE", section_type);
                let _ = fs::write(output_path, updated_data);

                // dbg!(&input_path);
                // dbg!(&output_path);
                // let _ = fs::copy(input_path, output_path);
            })
        });
    });
}


fn make_category_txt_files(_bounds: Vec<&str>, _config_dir: PathBuf, files: Vec<PathBuf>) {
    files.iter().for_each(|f| {
        let section_category = f.file_stem().unwrap();
        let file_data = fs::read_to_string(f).unwrap();
        let section_types: Vec<&str> = file_data.lines().collect();
        section_types.iter().for_each(|section_type| {
            let output_path =
                PathBuf::from(format!("../../sections/{}/category.txt", section_type));
            // dbg!(&output_path);
            let _ = fs::write(output_path, section_category.to_str().unwrap());
        });
    });
}


fn get_files_in_a_single_dir(dir: PathBuf) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .unwrap()
        .into_iter()
        .filter(|p| {
            if p.as_ref().unwrap().path().is_file() {
                true
            } else {
                false
            }
        })
        .filter_map(|p| match p.as_ref().unwrap().path().strip_prefix(".") {
            Ok(_) => None,
            Err(_) => Some(p.as_ref().unwrap().path()),
        })
        .collect()
}


fn get_files_in_dir_matching_extensions_recursively(dir: &PathBuf, exts: Vec<&str>) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter(|e| 
            match e.as_ref().unwrap().path().extension() {
                Some(x) => exts.contains(&x.to_str().unwrap()),
                None => false
    }).map(|e| e.unwrap().into_path()).collect()
}


fn make_parent_dir_for_file(file_path: &PathBuf) -> Result<String, String> {
    match file_path.parent() {
      Some(parent_dir) => match fs::create_dir_all(parent_dir) {
        Ok(_) => Ok("Made the dir".to_string()),
        Err(e) => Err(e.to_string())
      },
      None => Err("Could not make directory".to_string())
    }
  }
  
