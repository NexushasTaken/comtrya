use super::PackageProvider;
use crate::actions::package::repository::PackageRepository;
use crate::actions::package::PackageVariant;
use crate::atoms::command::Exec;
use crate::contexts::Contexts;
use crate::steps::Step;
use crate::utilities;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::process::Command;
use tracing::warn;
use tracing::{debug, trace};
use which::which;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pacman {}

impl PackageProvider for Pacman {
    fn name(&self) -> &str {
        "Pacman"
    }

    fn available(&self) -> bool {
        match which("pacman") {
            Ok(_) => true,
            Err(_) => {
                warn!(message = "pacman not available");
                false
            }
        }
    }

    fn bootstrap(&self, contexts: &Contexts) -> Vec<Step> {
        let privilege_provider =
            utilities::get_privilege_provider(contexts).unwrap_or_else(|| String::from("sudo"));

        vec![Step {
            atom: Box::new(Exec {
                command: String::from("pacman"),
                arguments: vec![
                    String::from("-S"),
                    String::from("--noconfirm"),
                    String::from("--needed"),
                    String::from("base"),
                ],
                privileged: true,
                privilege_provider: privilege_provider.clone(),
                ..Default::default()
            }),
            initializers: vec![],
            finalizers: vec![],
        }]
    }

    fn has_repository(&self, _: &PackageRepository) -> bool {
        false
    }

    fn add_repository(
        &self,
        _: &PackageRepository,
        _contexts: &Contexts,
    ) -> anyhow::Result<Vec<Step>> {
        Ok(vec![])
    }

    fn query(&self, package: &PackageVariant) -> anyhow::Result<Vec<String>> {
        let requested_already_installed: HashSet<String> = String::from_utf8(
            Command::new("pacman")
                .args(
                    vec![String::from("-Qq")]
                        .into_iter()
                        .chain(package.packages().into_iter()),
                )
                .output()?
                .stdout,
        )?
        .split('\n')
        .map(String::from)
        .collect();

        debug!(
            "all requested installed packages: {:?}",
            requested_already_installed
        );

        Ok(package
            .packages()
            .into_iter()
            .filter(|p| {
                if requested_already_installed.contains(p) {
                    trace!("{}: already installed", p);
                    false
                } else {
                    debug!("{}: doesn't appear to be installed", p);
                    true
                }
            })
            .collect())
    }

    fn install(&self, package: &PackageVariant, contexts: &Contexts) -> anyhow::Result<Vec<Step>> {
        let privilege_provider =
            utilities::get_privilege_provider(contexts).unwrap_or_else(|| String::from("sudo"));

        Ok(vec![Step {
            atom: Box::new(Exec {
                command: String::from("pacman"),
                arguments: [
                    vec![
                        String::from("-Sq"),
                        String::from("--needed"),
                        String::from("--noconfirm"),
                        String::from("--noprogressbar"),
                    ],
                    package.extra_args.clone(),
                    package.packages(),
                ]
                .concat(),
                privileged: true,
                privilege_provider: privilege_provider.clone(),
                ..Default::default()
            }),
            initializers: vec![],
            finalizers: vec![],
        }])
    }
}
