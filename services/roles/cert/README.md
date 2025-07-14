# Cert role

Creates certificates signed by the internal certificate authority. Depends on the `secrets/` subdirectory, which is syncronized and stored outside version control.

For more customizaition, internal CA secret paths can be set with `cert_options.ca.crt_path` and `cert_options.ca.key_path`. The defaults are derived from the base domain (`cert_options.base_domain`), and their locations are relative to the role in the repository.

Service name and user are required to be in the context in which the role is used, but the values in the context can be overridden using the `service` dictionary in `cert_options`.

## Arguments

```yml
service:
    name: service1
    user: user1

cert_options:
    service: # Override
        name: service1
        user: user1
    base_domain: example.org
    key:
        output: /tmp/cert_test/test.key
        # Optional
        owner: user1 # Default to service user
        group: user1 # Default to service user
        mode: a=,g=rw,u=rw
    crs:
        output: /tmp/cert_test/test.csr
        cn: example.org
        subject_alt_names:
            - DNS:subdomain.example.org
            - DNS:*.example.org
            - DNS:example.org
    crt:
        output: /tmp/cert_test/test.crt
        # Optional
        not_after: +90d
        not_before: -1d
        owner: user1 # Default to service user
        group: user1 # Default to service user
        mode: a=,g=rw,u=rw
    ca:
        crt_path: ../secrets/ca/example.org.crt
        key_path: ../secrets/ca/example.org.key
```
