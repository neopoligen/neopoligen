use neopoligen::builder::Builder;
use neopoligen::config::Config;
use neopoligen::file_set::FileSet;
// use neopoligen::site::Site;
// use std::path::PathBuf;

#[test]
pub fn basic_integration_solo() {
    let file_set = FileSet::set1();
    let config = Config::site1_config();
    let builder = Builder::new(file_set, &config);
    dbg!(builder.files_to_output());
    // let output_files = site.output_files();
    // dbg!(&output_files);
    // let _left = output_files.get(&PathBuf::from(
    //     "leading-dir/Neopoligen/dev-test-site-1/docs/index.html",
    // ));
}
