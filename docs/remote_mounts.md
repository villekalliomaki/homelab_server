# Hetzner Storage box

With a new storage box before configuration, the machine's SSH public key has to be imported to the storage box to make automatically mounting it easier. The storage box is accessible with a password, which can be used to import the SSH key. The SSH keys are copied to the server in `02-ssh.yml`. The SSH public key is copied to the storage box with `07-remote_mounts.yml`. Which also sets up a permanent mount to it in fstab.

If even root doen't have permissions to the mounted directory, either the remote mount path is wrong (is should be ./ or something under that), or maybe default_permissions/allow_other SSHFS options are the issue.
