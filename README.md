# Arch Linux based server installation

Directory structure:

-   `base`: Ansible playbooks to provisio and upkeep the server.
-   `services`: Ansible playbooks to run services with podman.
-   `containers`: Custom container build files.
-   `docs`: General documentation in markdown files.
-   `secrets`: Passphrases, certificates and keys not kept in version control. Synced seperately with Syncthing.

These can also have subdirectories, for example for common reusable groups of tasks.

## Manual preparation

Before running any automated installation steps, the Arch system has to be installed to a state where it is started with a SSH server running and Python installed.

1. Install Arch.
2. `pacman -S openssh less vim python`
3. Start SSH server: `systemctl enable --now sshd`
4. Copy authorized_keys to the root user of the server.
5. Add the server to the local SSH config with the hostname matching the inventory and the correct IP address.

## Running playbooks

Ansible inventory file is in the repository. The hostnames in it should be configured in the local machines SSH client's settings.

Example command at the repository root:

```sh
ansible-playbook --vault-password-file=secrets/ansible_vault_passphrase -i inventory base/01-packages.yml
```
