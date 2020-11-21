# Crablog

Pure rust. Built with actix, diesel, tera, serde and sqlite3.

## Build instructions using docker

1. Clone the repository
```bash
git clone https://github.com/leonardlorenz/crablog
cd crablog
```
2. Set up your configuration file (see below)
3. Build and run the docker container (Will compile from source and thus take a while)
```bash
docker-compose up -d
```

## Configuration environment file

All configuration options are defined in .env, to be created in the same directory as this readme.

An example configuration:

```
USERNAME=yourusername
EMAIL=me@mydomain.tld
BIND_PORT=8000
ROOT_PATH=/path/to/template/directory/and/sqliteDB
SUBMIT_TOKEN=Submit!123 # token needed for submitting
```
