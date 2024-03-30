# Networking

The default for podman v4.0 an newer is netavark which supports rootless containers. Dynamics addresses are fine to use, because Ansible gets the address dynamically on each run for the default interface and wg0.

Alpine container test command for pasta: `podman run -it --rm --network=pasta alpine sh`.

If a service consists of multiple containers in a single pod, only the pod should have it's network mode set to pasta. The containers will inherit the network configuration from the pod. If the containers have networks settings defined, they won't be able to comminnicate with each other.

# Rootless podman

All services have their own non-root user, which is used to run all the podman containers related to that service. Every service should be its own self contained system, unless it provides something to other services, like LDAP.

# Troubleshooting

## Error from `podman system migrate`

Something like "overlay is not supported over extfs at /var/lib/containers/storage/overlay" is solved by rebooting. Deleting ~/.config/containers can help too.
