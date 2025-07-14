# Redis role

Mostly based on the PostgreSQL role, but creates a Redis container to be used with some service. Utilizes variable `container_state` if it's available. Service name will be used for pod and container names if they are not specifically set.

Currently there is no way to run Redis just in memory, and a data volume path is always required.

Service name and user are required to be in the context in which the role is used, but the values in the context can be overridden using the `service` dictionary in `redis_options`.

## Arguments

Minimal:

```yml
service:
    name: service1
    user: user1

redis_options:
    password: redis_password
    paths:
        volume: /tmp/db1/data
        logs: /tmp/db1/log
```

All:

```yml
service:
        name: service1
        user: user1

redis_options:
    service: # Override
        name: service1
        user: user1
    password: redis_password
    paths:
        volume: /tmp/db1/data
        logs: /tmp/db1/log
    container_name: service_database1
    pod_name: service_pod
    image: docker.io/library/redis
    version: "8.0.2"
    recreate: false
    log_driver: k8s-file
    log_file: redis_container_log
    max_memory: 200mb
```

## Return values

The details of the created Redis container are saved to `redis_container_output`.
