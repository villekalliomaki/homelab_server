# PostgreSQL role

Create a PostgreSQL container to be used in a podman pod, related to some service. Requires directories for data and logs. Does use variable `container_state` if available, and otherwise defaults to a started state. Container, pod and database names will use the name of the service if nothing specific is defined.

Currently using PostgreSQL version 16 as the default, but can be set to anything else as well.

Service name and user are required to be in the context in which the role is used, but the values in the context can be overridden using the `service` dictionary in `postgresql_options`.

## Arguments

Minimal:

```yml
service:
        name: service1
        user: user1

postgresql_options:
    database:
        username: user
        password: abc123
        name: db_name
    paths:
        volume: /tmp/db1/data
        logs: /tmp/db1/log
```

All:

```yml
service:
        name: service1
        user: user1

postgresql_options:
    service: # Override
        name: service1
        user: user1
    database:
        username: user
        password: abc123
        name: db_name
    paths:
        volume: /tmp/db1/data
        logs: /tmp/db1/log
    container_name: service_database1
    pod_name: service_pod
    version: 16.2
    image: docker.io/library/postgres
```

## Return values

The results for the PostgreSQL container creation is saved to `postgresql_container_output`.
