# PostgreSQL role

Container name is service name with `_postgresql` appended. Depends on `global.yml`.

Role gets the target container state from the `container_state` variable. If it doesn't exit the state will default to `started`. The variable is not prefixed to avoid redefining it to comply with existing playbooks.

Almost always the container has to be inside a pod, which can be set using `postgresql_pod_name`. By defalt the pod used is the same as service name, which is also the default behaviour with the pod role.

A directory for container logs has to always be specified, and the name of the log file `postgresql_container_log` can't be changed to make it easier for promtail to find it.

Container creation output is saved to `postgresql_container_output`.

## Variables example

Required:

```yml
postgresql_volume_path: /tmp/postgresql_test
postgresql_log_path: /tmp/postgresql_logs

# Inherited from the service playbook
service:
    name: service_name
    user: username

# Database user
postgresql_user:
    username: user
    password: password
    database: db_name
```

Optional:

```yml
postgresql_container_image: docker.io/library/postgres:16.2

postgresql_recreate: false
```
