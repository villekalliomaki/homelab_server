# Directories role

By default only the base level volume and log directories are created. Subdirectories or cache have to be explicitly requested. Default base paths for all directories are defined, and can be changed. A name for the service and an username for the ownership of directories are required.

Service name and user are required to be in the context in which the role is used, but the values in the context can be overridden using the `service` dictionary in `directories_options`.

## Arguments

Minimal:

```yml
service:
    name: service1
    user: username1
```


All:

```yml
service:
    name: service1
    user: user1

directories_options:
    service: # Override
        name: service1
        user: username1
    volume:
        create: true
        subdirectories: []
        mode: a=,u=rwx,g=rwx
        recurse: true
        # Defaults to service user's name if null
        group: null
        base_path: /var/container_volumes
    logs:
        create: true
        subdirectories: []
        mode: a=,u=rwx,g=rwx
        recurse: true
        # Defaults to service user's name if null
        group: null
        base_path: /var/log/containers
    cache:
        create: false
        subdirectories: []
        mode: a=,u=rwx,g=rwx
        recurse: true
        # Defaults to service user's name if null
        group: null
        base_path: /mnt/cache
```

## Return values

Paths of the directories created can be accessed using variables defined by the role. For example the path for single volume directory is `directories_volume_output.path` and `directories_volume_subdirectories_output.results[0].path` for the first subdirectory.

Volumes:

-   `directories_volume_output`
-   `directories_volume_subdirectories_output`

Logs:

-   `directories_logs_output`
-   `directories_logs_subdirectories_output`

Cache:

-   `directories_cache_output`
-   `directories_cache_subdirectories_output`
