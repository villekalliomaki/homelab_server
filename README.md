# Server installation

Directory structure:

-   `base`: Ansible playbooks to provisio and upkeep the server.
-   `services`: Ansible playbooks to run services with podman.
-   `containers`: Custom container build files.
-   `docs`: General documentation in markdown files.
-   `secrets`: Passphrases, certificates and keys not kept in version control.
-   `backups`: Tool and deployment for automated remote backups.

## Manual preparation

Before running any automated installation steps, the Arch system has to be installed to a state where it is started with a SSH server running and Python installed.

1. Install Arch.
2. `pacman -S openssh less vim python`
3. Start SSH server: `systemctl enable --now sshd`
4. Copy authorized_keys to the root user of the server.
5. Add the server to the local SSH config with the hostname matching the inventory and the correct IP address.
