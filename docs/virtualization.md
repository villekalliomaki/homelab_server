# QEMU virtual machines

## Networking

The tap network backend is probably preferred in most cases. The guest IP address would probably be easiest to just get from DHCP, and with long enought lease times and no renewals, getting the IP address once and not dynamically updating it is probably enough.

## Persistent data

The vm disk image should not be the only place to store anything of value, since there is no point in backing up whole images, if it should be able to be recreated with Ansible.
