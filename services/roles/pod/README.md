# Pod role

Role gets the target container state from the `container_state` variable. If it doesn't exit the state will default to `started`. The creation of the pod will be done using whichever user is defined in the arguments, as this role should work with rooless services.

Service name and user are required to be in the context in which the role is used, but the values in the context can be overridden using the `service` dictionary in `pod_options`.

## Arguments

Minimal:

```yml
service:
    name: service1
    user: user1
```

All:

```yml
service:
    name: service1
    user: user1

pod_options:
    # Override
    service:
        name: service1
        user: user1
    # Optional
    user_namespace: ""
    network:
        - pasta
    publish:
        - 127.0.0.1:80:80
```
