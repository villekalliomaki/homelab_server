# Cron restart role

## Arguments

Name should be unique for each service user. Containers can be a list or a single entry. Shutdown grace time is in seconds.

Service name and user are required to be in the context in which the role is used, but the values in the context can be overridden using the `service` dictionary in `cron_restart_options`.

```yml
service:
    name: service1
    user: username1

cron_restart_options:
    service: # Override
        name: service1
        user: username1
    # Required
    identifier: container_restart_job
    containers:
        - server
        - postgresql
    # Optional
    time:
        minute: "*"
        hour: "3"
        day: "*"
        month: "*"
        weekday: "*"
    grace: 120
```
