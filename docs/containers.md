# Networking

The default for podman v4.0 an newer is netavark which supports rootless containers. Dynamics addresses are fine to use, because Ansible gets the address dynamically on each run for the default interface and wg0.

Alpine container test command for pasta: `podman run -it --rm --network=pasta alpine sh`.

If a service consists of multiple containers in a single pod, only the pod should have it's network mode set to pasta. The containers will inherit the network configuration from the pod. If the containers have networks settings defined, they won't be able to comminnicate with each other.

# Naming

All playbooks directly in `./services` have to be the same name as the service user, because the file names are used to generate scripts and other playbooks.

# Monitoring

A podman exporter instance is created to all service users, with the ports being generated from the list of service playbook files.

# Troubleshooting

### Error from `podman system migrate`

Something like "overlay is not supported over extfs at /var/lib/containers/storage/overlay" is solved by rebooting. Deleting ~/.config/containers can help too.

### AttributeError: 'list' object has no attribute 'get'

For some reason Ansible doesn't work like it should with pods that already exist. Easiest way to fix the error is to stop and remove all containers in a service, and then run the setup playbook again.

### Permission errors inside container

Some of these could work:

-   Map the real user to something else inside the container, ofter to "nobody" with `userns: keep-id:uid=65534,gid=65534`
-   Make sure files or directories have the execution permission added, if for some reason the container check and errors from it being missing.

Permission issues is container volume directories are usually from too limited permissions of the parents directories. Now this should be taken into account in `08-directories.yml`.

### Error `AttributeError: 'list' object has no attribute 'get'` when starting a service

If the containers related to a pod are stopped or deleted without deleting the pod, manually or in a script, Ansible will throw an error when trying to create the existing pod. (probably)
