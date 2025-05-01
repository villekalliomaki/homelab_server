use std::process::exit;

use anyhow::Error;
use log::{error, info};
use rustic_core::repofile::SnapshotSummary;
use sailfish::TemplateOnce;

use crate::{
    args::Args, backup::create_snapshot, config::Config, email::Email, templates::ReportTemplate,
};

mod args;
mod backup;
mod config;
mod email;
mod templates;

fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    info!("Starting backup runner");

    let arguments = Args::generate();
    let config = Config::from_file(&arguments.config_file);

    let mut summaries: Vec<(String, SnapshotSummary)> = Vec::new();
    let mut errors: Vec<(String, Error)> = Vec::new();

    for backup in config.backups {
        info!("Starting new backup snapshot");

        // Create snapshot
        let backup_result = create_snapshot(
            backup.source_paths,
            backup.endpoint,
            backup.repo_path,
            backup.ssh_key_path,
            backup.user,
            backup.repo_password,
        );

        let snapshot = match backup_result {
            Ok(b) => b,
            Err(error) => {
                error!(
                    "Backup snapshot creation failed for {}: {}",
                    backup.name, error
                );

                errors.push((backup.name.clone(), error));

                break;
            }
        };

        match snapshot.0.summary {
            Some(summary) => summaries.push((backup.name, summary)),
            None => {
                error!(
                    "Snapshot summary missing for {} ({})",
                    backup.name, snapshot.0.id
                );

                errors.push((
                    backup.name.clone(),
                    Error::msg(format!(
                        "Snapshot summary missing for {} ({})",
                        backup.name, snapshot.0.id
                    )),
                ));

                continue;
            }
        };

        info!("Successful backup snapshot {} created", snapshot.0.id);
    }

    // Compile and send email

    let subject: String;

    if errors.is_empty() {
        subject = format!("Backup report: {} OK", summaries.len());
    } else {
        subject = format!(
            "({} FAILED): Backup report: {} FAILED, {} ok",
            errors.len(),
            errors.len(),
            summaries.len(),
        );
    }

    let context = ReportTemplate { summaries, errors };

    info!("Rendering email report body");

    let content = match context.render_once() {
        Ok(s) => s,
        Err(error) => {
            error!("Template rendering error: {}", error);
            exit(1);
        }
    };

    match Email::new(&config.email_settings)
        .subject(subject)
        .content(content)
        .send()
    {
        Ok(_) => {
            info!("Email report sent");
        }
        Err(error) => {
            error!("Error while sending email report: {}", error);
            exit(1);
        }
    }
}
