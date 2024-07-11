# Pod role

Role gets the target container state from the `container_state` variable. If it doesn't exit the state will default to `started`. The variable is not prefixed to avoid redefining it to comply with existing playbooks.

To run the pod as a rootless user, the role needs a username to change to, and to keep variable names as same as possible service name is also set with a variable without a prefix:

## Variables example

```yml
service:
    name: service_name
    user: username

pod_publish:
    - "127.0.0.1:80:80"
```
