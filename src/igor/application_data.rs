use std::{env, path::PathBuf};

use crate::AnyResult;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ApplicationData {
    pub current_directory: PathBuf,
    pub project_name: String,
}

impl ApplicationData {
    pub fn new(destination_yyp: &Option<String>) -> AnyResult<Self> {
        let current_directory =
            env::current_dir().map_err(|_| color_eyre::eyre::anyhow!("cannot read directory"))?;

        let mut project_name = None;
        let mut too_many_projects = vec![];

        for file in current_directory.read_dir()?.flatten() {
            let file = file.path();

            if let Some(ext) = file.extension() {
                if ext == "yyp" {
                    let stem = file.file_stem().unwrap().to_string_lossy().to_string();
                    if too_many_projects.is_empty() {
                        if let Some(dest) = destination_yyp {
                            if stem == *dest {
                                project_name = Some(stem);
                            }
                        } else if let Some(project_name) = project_name.take() {
                            too_many_projects.push(project_name);
                            too_many_projects.push(stem);
                        } else {
                            project_name = Some(stem);
                        }
                    } else {
                        too_many_projects.push(stem);
                    }
                }
            }
        }

        if too_many_projects.is_empty() == false {
            color_eyre::eyre::bail!(
                "multiple yyps discovered. specify target with --target. options: \n\
            \t{}",
                too_many_projects
                    .into_iter()
                    .fold(String::new(), |mut accum, v| {
                        if accum.is_empty() == false {
                            accum.push(',');
                            accum.push(' ');
                        }
                        accum.push_str(&v);

                        accum
                    })
            );
        }

        if let Some(project_name) = project_name {
            Ok(Self {
                current_directory,
                project_name,
            })
        } else {
            color_eyre::eyre::bail!(
                "could not find a valid yyp in {}",
                current_directory.display()
            )
        }
    }
}
