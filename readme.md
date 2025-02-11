# tu2i

Turso Usage to InfluxDB

## Why ?

I use [Turso](https://turso.tech/), and I want to get alerts for my quota usage.
So this tool requests the Turso platform API to get the usage then sends it
to InfluxDB. (Where I later use Grafana to alert me if it goes to high).

## Build

```bash
# with cargo
❯ cargo build --release

# with nix
❯ nix build
# or
❯ nix build .#tu2i

# with nix for a docker image
❯ nix build .#dockerImage && docker load < result
```

## Run

```bash
# inline
❯ TURSO_API_TOKEN="<your-platform-api-token>" \
INFLUX_DB_URL="http://localhost:8086/api/v2/write?org=YOUR_ORG&bucket=YOUR_BUCKET&precision=ms" \
INFLUX_DB_TOKEN="<your-token>" \
./result/bin/tu2i

# with a .env
❯ cp .env.example .env
❯ ./result/bin/tu2i

# with docker
docker run --name tu2i -d \
-e TURSO_API_TOKEN="<your-platform-api-token>" \
-e INFLUX_DB_URL="http://localhost:8086/api/v2/write?org=YOUR_ORG&bucket=YOUR_BUCKET&precision=ms" \
-e INFLUX_DB_TOKEN="<your-token>" \
tu2i
```

## Extras

By default, this reports every 5 minutes. You can change this with the `EVERY_SEC` environment variable.
