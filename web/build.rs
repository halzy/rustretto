use std::path::PathBuf;

use grass;

fn main() {
    // watch the rust src code
    let src_dir: PathBuf = [env!("CARGO_MANIFEST_DIR"), "src"].iter().collect();
    println!("cargo:rerun-if-changed={}", src_dir.to_string_lossy());

    // Watch the web/scss dir for changes
    let scss_dir: PathBuf = [env!("CARGO_MANIFEST_DIR"), "scss"].iter().collect();
    println!("cargo:rerun-if-changed={}", scss_dir.to_string_lossy());

    // Our entry point for the scss styling
    let mut scss_app = scss_dir.clone();
    scss_app.push("app.scss");

    println!("app location: {:?}", &scss_app.to_string_lossy());

    // Convert SCSS to CSS
    let css = grass::from_path(scss_app, &grass::Options::default()).expect("can build scss");

    // Write the CSS out to the assets directory
    let css_output: PathBuf = [env!("CARGO_MANIFEST_DIR"), "..", "assets", "app.css"]
        .iter()
        .collect();
    std::fs::write(css_output, &css).expect("Can write SCSS to CSS");
}
