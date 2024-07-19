# Redis role

Identical to the PostgreSQL role, exept for Redis related variables. Only supports rootless containers.

Container name is service name with `_redis` appended. Depends on `global.yml`.

Role gets the target container state from the `container_state` variable. If it doesn't exit the state will default to `started`. The variable is not prefixed to avoid redefining it to comply with existing playbooks.

Almost always the container has to be inside a pod, which can be set using `redis_pod_name`. By defalt the pod used is the same as service name, which is also the default behaviour with the pod role.

A directory for container logs has to always be specified, and the name of the log file `redis_container_log` can't be changed to make it easier for promtail to find it.

Container creation output is saved to `redis_container_output`.

## Persistent data

By default Redis runs completely in memory, so everything deleted when the container is restarted. To save redis data to the disk, just provide the full path to a directory using `redis_data_path`. **WARNING: path is not validated in any way, and mode or ownership can be changed!**

## Variables example

```yml
redis_log_path: /tmp/redis_logs
# Optional for persistent data
redis_data_path: /tmp/redis_data

# Inherited from the service playbook
service:
    name: service_name
    user: username

# Redis user
redis_user:
    username: user
    password: password

# Optional container config
redis_container_image: docker.io/library/redis:7.2
redis_recreate: false
```
