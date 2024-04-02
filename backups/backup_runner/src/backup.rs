use anyhow::{Ok, Result};
use log::info;
use rustic_backend::BackendOptions;
use rustic_core::{
    repofile::SnapshotFile, BackupOptions, PathList, Repository, RepositoryBackends,
    RepositoryOptions, SnapshotOptions,
};

// Generate backend based on the target repository path/uri/url
fn get_backends(target_repository: String) -> Result<RepositoryBackends> {
    info!("Creating backend for {}", target_repository);

    BackendOptions::default()
        .repository(target_repository)
        .to_backends()
}

// Collect repository options
fn get_repo_options(password: String) -> RepositoryOptions {
    info!("Creating repository options");

    RepositoryOptions::default().password(password)
}

fn paths_list_from_string(paths: String) -> Result<PathList> {
    info!("Parsing list of paths: {}", paths);

    Ok(PathList::from_string(&paths)?.sanitize()?)
}

pub fn create_snapshot(
    sources: String,
    target_repository: String,
    password: String,
) -> Result<SnapshotFile> {
    info!(
        "Starting snapshot from {} to {}",
        sources, target_repository
    );

    // Set up
    let options = get_repo_options(password);
    let backends = get_backends(target_repository)?;
    let snapshot = SnapshotOptions::default().to_snapshot()?;

    // Try to open repository
    let repo = Repository::new(&options, backends)?
        .open()?
        .to_indexed_ids()?;

    info!("Created and opened repository");

    // Parse sources list
    let parsed_sources = paths_list_from_string(sources)?;
    info!("Backup source paths parsed as: {}", parsed_sources);

    Ok(repo.backup(&BackupOptions::default(), &parsed_sources, snapshot)?)
}
