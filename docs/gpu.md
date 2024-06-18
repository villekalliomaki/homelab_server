# Nvidia drivers

# GPU passthrough to podman containers

?

# Driver upgrades and Nvidia container toolkit

Error `Error: setting up CDI devices: failed to inject devices: failed to stat CDI host device /dev/nvidia-uvm` is resolved by deleting `/etc/cdi/nvidia.yaml` and then regenerating it with `/usr/bin/nvidia-ctk cdi generate --output=/etc/cdi/nvidia.yaml`. Cause is most likely that during some combination of pacman package upgrades the cdi spec is generated, but is using using replaced in the same pacman upgrade.
