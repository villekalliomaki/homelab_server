# Service user role

## Variables example

User is removed from all other groups (excluding the default) and added to the speficied ones. If no groups are specified all but the default are removed.

```yml
service:
    # Name is not required, but can be set
    # name: service
    user: username

service_user_groups:
    - group_name
```

## Return values

The start of the subuid/subgid range is returned with `service_user_subuid_start` (not tested to actually be correct). The service user task output is stored to `service_user_output`, the UID being `service_user_output.uid`. 
