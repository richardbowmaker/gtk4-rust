
use std::env;
use std::process::Command;

fn main() {

    // this method needs to be inside main() method
    // env::set_var("RUST_BACKTRACE", "1");

    // command line equivalent (see https://manpages.ubuntu.com/manpages/trusty/en/man1/glib-compile-resources.1.html)
    // glib-compile-resources --sourcedir="../" --target="../main_window_gtk4.gresource" "../main_window_gtk4.gresource.xml"
    // the target will be written to the OUT_DIR directory by this build script
    // compiles a binary gresource file from the content of the gresource.xml file.
    glib_build_tools::compile_resources(
        &["./resources"],
        "./resources/resources.gresource.xml",
        "resources.gresource",
    ); 

}

