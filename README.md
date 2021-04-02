# Crablog

Pure rust. Built with actix, diesel, tera, serde and sqlite3.

## Build instructions using docker

1. Clone the repository
```bash
git clone https://github.com/leonardlorenz/crablog
cd crablog/site
```
2. Install diesel and create a database
```bash
cargo install diesel_cli --no-default-features --features "sqlite"
diesel setup --database-url ../content/db.sqlite3
diesel migration run --database-url ../content/db.sqlite3
```
3. Set up your configuration file (see below)
4. Build and run the docker container (Will compile from source and thus take a while)
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
SUBMIT_TOKEN=Submit!123 # token needed for submitting
GITHUB_ACCOUNT=usernam3
TWITTER_ACCOUNT=usernam3
MASTODON_ACCOUNT=usernam3@mastodon.social
REDDIT_ACCOUNT=usernam3
DISCORD_ACCOUNT=usernam3

# only needed when not using a docker container
ROOT_PATH=/path/to/template/directory/and/sqliteDB
```

## Routes

- / site welcome
- /blog shows the last 5 posts
- /blog/id/<id> shows a single post by id
- /blog/all shows all posts
- /blog/submit set your submit token and create posts
- /blog/edit/<id> edit, delete or hide posts
  
**API Routes**

- /api/blog/posts returns all posts as json
