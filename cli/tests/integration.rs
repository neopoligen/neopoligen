use neopoligen::config::Config;
use neopoligen::file_set::FileSet;
use neopoligen::site::Site;
use std::path::PathBuf;

#[test]
pub fn basic_integration() {
    let config = Config::site1_config();
    let file_set = FileSet::set1();
    let site = Site::new(&config, file_set);
    dbg!(&site);
    let output_files = site.output_files();
    let left = output_files.get(&PathBuf::from(
        "leading-dir/Neopoligen/dev-test-site-1/docs/index.html",
    ));
    dbg!(left);
}
