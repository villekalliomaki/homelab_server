# PostgreSQL role

Container name is service name with `_postgresql` appended. Podman default settings container log path are required, so this depends on `global.yml`.

Role gets the target container state from the `container_state` variable. If it doesn't exit the state will default to `started`. The variable is not prefixed to avoid redefining it to comply with existing playbooks.

Almost always the container has to be inside a pod, which can be set using `postgresql_pod_name`. By defalt the pod used is the same as service name.

Container creation output is saved to `postgresql_container_output`.

## Variables example

Required:

```yml
postgresql_volume_path: /tmp/postgresql_test

service:
    name: service_name
    user: username

# Database user
postgresql:
    username: user
    password: password
    database: db_name
```
