# Custom nginx build

Adds the ngx_http_geoip2_module to nginx to support traffic control by country.

Mostly sourced from [this](https://sabbir.dev/article/deploy-geo-restriced-service-with-nginx-geoip2-and-docker/).

# Building

To build and push:

```sh
docker buildx build --builder default --platform linux/amd64 --push -t villekalliomaki/nginx:latest .
```

To build with fresher packages, clear build cache before: `docker buildx prune -a`.

# Troubleshooting

### ERROR: failed to solve: dockerfile parse error on line 10: unknown instruction: apt-get

Invalid formatting, probably backslah line continue broken by automatic formatting.
