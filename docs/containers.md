# Networking

The default for podman v4.0 an newer is netavark which supports rootless containers. Dynamics addresses are fine to use, because Ansible gets the address dynamically on each run for the default interface and wg0.

Alpine container test command for pasta: `podman run -it --rm --network=pasta alpine sh`

# Rootless podman

All services have their own non-root user, which is used to run all the podman containers related to that service. Every service should be its own self contained system, unless it provides something to other services, like LDAP.

# Troubleshooting

## Error from `podman system migrate`

Something like "overlay is not supported over extfs at /var/lib/containers/storage/overlay" is solved by rebooting. Deleting ~/.config/containers can help too.
