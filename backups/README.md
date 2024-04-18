# Automated backups

Single backup program which can have multiple pairs of sources and destination targets. All backups are encrypted with restic. Reports from all backups are compiled to a single email alert.

# Starting backup runner

The target repository has to be initialized before running, with the same password that is given in the config file. By default the program will use a `backup_runner.toml` file in the working directory, but it can the changed with the --config-file or -c flags. If the given file is not found the program will exit.

Initialize a local repository: `restic init --repo /tmp/backup_test/repo`

Initialize a remote repository: `restic -r sftp:foo:/srv/restic-repo init`

If the password has already been encrypted with ansible-vault, it will be printed when provisioning the backups.

# Backup runner config template

```toml
[email_settings]
from = "Backup Runner <noreply@example.org>"
to = "Notification Recipient <noreply@example.org>"
host = "mail.example.org"
port = 465
username = "noreply@example.org"
password = "12345"

[[backups]]
name = "test1"
source_paths = "/tmp/backup_test/source1 /tmp/backup_test/source2"
target_repo = "/tmp/backup_test/repo"
repo_password = "123123123"

```
