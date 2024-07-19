# Directories role

**Depends on `global.yml`.**

## Minimal required arguments

Does not create anything, but nothing else is required to make role as flexible as possible.

```yml
service:
    name: service_name
    user: username
```

## Container volume

Volume directory is named after service. With just `create: true` the base container volume will be created using the default settings. If `create` is not set to true, nothing is done even if other settings are defined. Group will default to the user's primary group if not defined, which is just the username.

```yml
directories_container_volume:
    # Required to be created
    create: true
    # Optional from here
    subdirectories: []
    mode: a=,u=rwx,g=rwx
    recurse: true
    group: some_group
```

Return values:

-   `dirs_container_volume`
-   `dirs_container_volume_subdirectories`

## Container logs

Identical to container volume, but can't be disabled. Subdirectories are optional.

```yml
directories_container_logs:
    # Optional from here
    subdirectories: []
    mode: a=,u=rwx,g=rwx
    recurse: true
    group: some_group
```

Return values:

-   `dirs_container_logs`
-   `dirs_container_logs_subdirectories`

## Container cache

Also the same.

```yml
directories_container_cache:
    # Required to be created
    create: true
    # Optional from here
    subdirectories: []
    mode: a=,u=rwx,g=rwx
    recurse: true
    group: some_group
```

Return values:

-   `dirs_container_cache`
-   `dirs_container_cache_subdirectories`
