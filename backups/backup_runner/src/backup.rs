use anyhow::{Ok, Result};
use log::info;
use rustic_backend::BackendOptions;
use rustic_core::{
    repofile::SnapshotFile, BackupOptions, KeepOptions, PathList, PruneOptions, Repository,
    RepositoryBackends, RepositoryOptions, SnapshotGroupCriterion, SnapshotOptions,
};
use std::collections::BTreeMap;

// Generate backend based on the target repository path/uri/url
fn get_backends(
    endpoint: String,
    repo_path: String,
    ssh_key_path: String,
    user: String,
) -> Result<RepositoryBackends> {
    info!("Creating backend for {}", endpoint);

    // Create options for SFTP repository
    let mut options = BTreeMap::new();

    options.insert("endpoint".to_string(), endpoint);
    options.insert("user".to_string(), user);
    options.insert("key".to_string(), ssh_key_path);
    options.insert("root".to_string(), repo_path);

    Ok(BackendOptions::default()
        .repository("opendal:sftp")
        .options(options)
        .to_backends()?)
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
    endpoint: String,
    repo_path: String,
    ssh_key_path: String,
    no_history: bool,
    user: String,
    password: String,
) -> Result<SnapshotFile> {
    info!("Starting snapshot from {} to {}", sources, endpoint);

    // Set up
    let options = get_repo_options(password);
    let backends = get_backends(endpoint, repo_path, ssh_key_path, user)?;
    let snapshot = SnapshotOptions::default().to_snapshot()?;

    // Try to open repository
    let repo = Repository::new(&options, &backends)?
        .open()?
        .to_indexed_ids()?;

    info!("Created and opened repository");

    // Parse sources list
    let parsed_sources = paths_list_from_string(sources)?;
    info!("Backup source paths parsed as: {}", parsed_sources);

    let backup_result = repo.backup(&BackupOptions::default(), &parsed_sources, snapshot)?;

    // Forget snapshots based on criteria
    let group_by = SnapshotGroupCriterion::default();

    let mut keep = KeepOptions::default();

    if no_history {
        // Just keep one daily
        keep = keep.keep_daily(1);
    } else {
        // Normal prune
        keep = keep
            .keep_daily(30)
            .keep_weekly(12)
            .keep_monthly(12)
            .keep_yearly(5);
    }

    // Forget snapshots
    let forget_snapshots_ids = repo
        .get_forget_snapshots(&keep, group_by, |_| true)?
        .into_forget_ids();

    // Then delete the list of snapshots
    repo.delete_snapshots(&forget_snapshots_ids)?;

    // Then prune repo to acutally delete old snapshot data
    // (And delete instantly)
    let prune_options = PruneOptions::default().instant_delete(true);

    repo.prune(&prune_options, repo.prune_plan(&prune_options)?)?;

    Ok(backup_result)
}
