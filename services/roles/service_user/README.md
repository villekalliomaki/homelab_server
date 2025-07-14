# Service user role

User is removed from all other groups (excluding the default) and added to the speficied ones. If no groups are specified, all but the default are removed.

Service name and user are required to be in the context in which the role is used, but the values in the context can be overridden using the `service` dictionary in `service_user_options`.

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
    user: username1

service_user_options:
    service: # Override
        name: service1
        user: username1
    # Optional
    groups: ["media"]
```

## Return values

The start of the subuid/subgid range is returned with `service_user_subuid_start` (not tested to actually be correct). The service user task output is stored to `service_user_output`, the UID being `service_user_output.uid`.
