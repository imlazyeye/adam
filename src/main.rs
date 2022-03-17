#![allow(clippy::bool_comparison)]
#![deny(rust_2018_idioms)]
#![deny(rust_2021_compatibility)]

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
compile_error!("we only support `windows` and `macos` targets!");

use crate::igor::{OutputKind, TargetFolders};

type AnyResult<T = ()> = color_eyre::eyre::Result<T>;

mod input {
    mod cli;
    mod config_file;
    mod get_input;
    pub use get_input::{parse_inputs, Operation, RunKind};
}
mod igor {
    mod application_data;
    pub use application_data::*;

    mod build;
    pub use build::*;
}

mod gm_artifacts;
pub use gm_artifacts::{DefaultPlatformData, DEFAULT_PLATFORM_DATA, DEFAULT_RUNTIME_NAME};
mod manifest;

mod runner;
pub use runner::{PlatformOptions, RunOptions, TaskOptions};

#[cfg(target_os = "windows")]
mod trailing_comma_util;

fn main() -> AnyResult {
    color_eyre::install()?;

    let (mut options, operation) = input::parse_inputs()?;
    if let Err(e) = options.platform.canonicalize() {
        println!(
            "{}: {}",
            console::style("adam error").bright().red(),
            console::style(e).bold()
        );

        return Ok(());
    }
    if options.task.yyc {
        if let Err(e) = options.platform.canonicalize_yyc() {
            println!(
                "{}: {}",
                console::style("adam error").bright().red(),
                console::style(e).bold()
            );

            return Ok(());
        }
    }

    let application_data = match igor::ApplicationData::new() {
        Ok(v) => v,
        Err(e) => {
            println!(
                "{}: {}",
                console::style("adam error").bright().red(),
                console::style(e).bold()
            );

            return Ok(());
        }
    };

    // handle a clean, extract the build_data
    let run_kind = match operation {
        input::Operation::Run(inner) => inner,
        input::Operation::Clean => {
            // clean up the output folder...
            if let Err(e) = std::fs::remove_dir_all(
                application_data
                    .current_directory
                    .join(&options.task.output_folder),
            ) {
                println!("{} on clean: {}", console::style("error").bright().red(), e);
            }
            return Ok(());
        }
    };

    // fire any specific behavior to this run kind
    if run_kind == input::RunKind::Test {
        for var in options.task.test_env_variables.iter() {
            std::env::set_var(var, "1");
        }
    }

    // check if we have a valid yyc bat
    if options.task.yyc {
        if cfg!(not(target_os = "windows")) {
            println!(
                "{}: {}\nPlease log a feature request at https://github.com/NPC-Studio/adam/issues",
                console::style("adam error",).bright().red(),
                console::style("adam does not support macOS YYC compilation, yet.").bold(),
            );
            return Ok(());
        }

        if options.platform.visual_studio_path.exists() == false {
            println!(
                "{}: {}.\n\
            Supplied path in preferences was \"{}\" but it did not exist.\n\
            To use yyc, we must have a visual studio .bat file.\n\
        Please specify a path in the Gms2 IDE. \n\
        For more information, see \
        https://help.yoyogames.com/hc/en-us/articles/227860547-GMS2-Required-SDKs",
                console::style("error").bright().red(),
                console::style("no valid path to visual studio .bat build file").bold(),
                options.platform.visual_studio_path,
            );

            return Ok(());
        }
    }

    let output_kind = if options.task.yyc {
        igor::OutputKind::Yyc
    } else {
        igor::OutputKind::Vm
    };

    let build_data = igor::BuildData {
        folders: TargetFolders::new(
            &application_data.current_directory,
            options.task.output_folder.as_std_path(),
            output_kind,
            &application_data.project_name,
        )?,
        output_kind,
        project_filename: application_data.project_name,
        project_directory: application_data.current_directory,
        // user_dir: options.platform.user_data.clone(),
        user_dir: Default::default(),
        license_folder: options
            .platform
            .user_license_folder
            .as_std_path()
            .to_owned(),
        runtime_location: options.platform.runtime_location.as_std_path().to_owned(),
        // target_mask: options.platform.target_mask,
        target_mask: 0,
        application_path: options
            .platform
            .gms2_application_location
            .as_std_path()
            .to_owned(),
        config: options.task.config.clone(),
    };

    let gm_build = gm_artifacts::GmBuild::new(&build_data);
    let macros = gm_artifacts::GmMacros::new(&build_data);
    let visual_studio_path = options.platform.visual_studio_path.clone();

    // check if we need to make a new build at all, or can go straight to the runner
    if options.task.ignore_cache == 0
        && manifest::check_manifest(
            build_data.config.clone(),
            &build_data.project_directory,
            &build_data.folders.cache,
            &build_data.folders.main,
        )
    {
        runner::rerun_old(gm_build, &macros, options);
        return Ok(());
    }

    // clear the temp files...
    build_data.folders.clear_tmp()?;

    let build_location = build_data.folders.cache.join("build.bff");

    // write in the build.bff
    std::fs::write(
        &build_location,
        serde_json::to_string_pretty(&gm_build).unwrap(),
    )
    .unwrap();

    // write in the preferences
    let preferences = if build_data.output_kind == OutputKind::Yyc {
        gm_artifacts::GmPreferences::new(visual_studio_path.as_std_path().to_owned())
    } else {
        gm_artifacts::GmPreferences::default()
    };
    std::fs::write(
        &gm_build.preferences,
        serde_json::to_string_pretty(&preferences).unwrap(),
    )
    .unwrap();

    // write in the targetoptions
    std::fs::write(
        &gm_build.target_options,
        serde_json::to_string_pretty(&gm_artifacts::GmTargetOptions {
            runtime: build_data.output_kind,
        })
        .unwrap(),
    )
    .unwrap();

    // write in the steamoptions -- we just use defaults here...
    std::fs::write(
        &gm_build.steam_options,
        serde_json::to_string_pretty(&gm_artifacts::GmSteamOptions::default()).unwrap(),
    )
    .unwrap();

    // and we write the macros finally
    std::fs::write(
        &gm_build.macros,
        serde_json::to_string_pretty(&macros).unwrap(),
    )
    .unwrap();

    if runner::run_command(&build_location, macros, options, run_kind) {
        Ok(())
    } else {
        Err(color_eyre::eyre::eyre!("adam failed"))
    }
}
