# Directories role

## Ouputs

-   Volume
    -   `directories_output_volume`
    -   `directories_output_volume_subdirectories`
-   Container logs
    -   `directories_output_container_logs`
    -   `directories_output_container_logs_subdirectories`
-   Cache
    -   `directories_output_cache`
    -   `directories_output_cache_subdirectories`

## Minimal required arguments

Creates the volume and container logs directories named after the user. Subdirectories or cache is not created.

```yaml
directories_permissions:
    user: owner_username
```

## All arguments (with defaults):

```yaml
directories_base_paths:
    volume: /var/container_volumes
    container_logs: /var/log/containers
    cache: /mnt/cache

directories_permissions:
    user: null
    # Defaults to same as user
    group: null

directories_volume:
    create: true
    # Defaults to username
    name: service_name
    subdirectories: []
    mode: a=,u=rwx,g=rwx
    recurse: true

directories_container_logs:
    create: true
    # Defaults to username
    name: service_name
    subdirectories: []
    mode: u=rwx,g=rwx
    recurse: true

directories_cache:
    create: user
    # Defaults to username
    name: service_name
    subdirectories: []
    mode: u=rwx,g=rwx
    recurse: true
```
