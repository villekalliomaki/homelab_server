# Cert role

Creates certificates signed by the internal certificate authority. Depends on the `secrets/` subdirectory, which is syncronized and stored outside git version control. Limited to creating only certs with second level or more based of the base domain, so this also depends on `global.yml`.

For further customizaition internal CA secret paths can be set with `cert_ownca_path` and `cert_ownca_privatekey_path`. The defaults are derived from the base domain, and their locations are relative to the role in the repository.

# Variables

```yml
cert_key_options:
    output: /tmp/cert_test/test.key
    # Optional (with defaults)
    owner: username # defaults to service.user
    group: groupname # defaults to service.user
    mode: u=rw

cert_csr_options:
    output: /tmp/cert_test/test.csr
    common_name: example.org
    # Can also be an empty list or missing is none
    subject_alt_names:
        - "subdomain.example.org"
        - "*.example.org"
        - "example.org"

cert_crt_options:
    output: /tmp/cert_test/test.crt
    # Optional (with defaults)
    not_after: +90d
    not_before: -1d
    owner: username # defaults to service.user
    group: groupname # defaults to service.user
    mode: u=rw
```
