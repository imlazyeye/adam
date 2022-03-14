use crate::gm_artifacts;
use std::{path::Path, process::Child};

use super::run::RunCommand;

pub fn invoke_run(
    macros: &gm_artifacts::GmMacros,
    build_bff: &Path,
    sub_command: &RunCommand,
) -> Child {
    let mut igor = std::process::Command::new(gm_artifacts::MONO_LOCATION);
    igor.arg(macros.igor_path.clone())
        .arg("-j=8")
        .arg(format!("-options={}", build_bff.display()));

    // add the verbosity
    if sub_command.1.task.verbosity > 1 {
        igor.arg("-v");
    }

    let command = format!(
        "{} -j=8 -options={} -v -- {} PackageZip",
        macros.igor_path.clone().to_str().unwrap(),
        build_bff.display(),
        gm_artifacts::PLATFORM_KIND.to_string(),
    );
    println!("{command}");

    // add the platform
    igor.arg("--")
        .arg(gm_artifacts::PLATFORM_KIND.to_string())
        .arg("Run")
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap()
}

pub fn invoke_release(
    macros: &gm_artifacts::GmMacros,
    build_bff: &Path,
    sub_command: &RunCommand,
) -> Child {
    let mut igor = std::process::Command::new(gm_artifacts::MONO_LOCATION);
    igor.arg(macros.igor_path.clone())
        .arg("-j=8")
        .arg(format!("-options={}", build_bff.display()));

    // add the verbosity
    if sub_command.1.task.verbosity > 1 {
        igor.arg("-v");
    }

    igor.arg(format!(
        "--lf={}",
        macros.user_directory.join("licence.plist").display()
    ));

    // add the platform
    igor.arg("--")
        .arg(gm_artifacts::PLATFORM_KIND.to_string())
        .arg("PackageZip");

    igor.stdout(std::process::Stdio::piped()).spawn().unwrap()
}

pub fn invoke_rerun(gm_build: &gm_artifacts::GmBuild) -> Child {
    std::process::Command::new(
        gm_build
            .runtime_location
            .join("mac/YoYo Runner.app/Contents/MacOS/Mac_Runner"),
    )
    .arg("-game")
    .arg(gm_build.output_folder.join("GameAssetsMac.zip"))
    .arg("-debugoutput")
    .arg(gm_build.output_folder.join("debug.log"))
    .arg("-output")
    .arg(gm_build.output_folder.join("debug.log"))
    .arg("-runTest")
    .stdout(std::process::Stdio::piped())
    .stderr(std::process::Stdio::null())
    .spawn()
    .unwrap()
}
