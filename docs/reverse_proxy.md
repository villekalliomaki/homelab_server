# Troubleshooting

## nginx always returns 403, and Authelia is not recieving any requests

The nginx config probably doesn't allow the used local IP address range in the geoblock section. When testing with a VM the subnet can be something complete different than the usual ones in production. Using slirp4netns with rootless containers doesn't forward the source ip, so geoblocks don't work as it uses an internal podman network address.
