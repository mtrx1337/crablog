# Crablog

Pure rust. Built with actix, diesel, tera, serde and sqlite3.

## Build instructions using docker

1. Clone the repository
```bash
git clone https://github.com/mtrx1337/crablog
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

All configuration options are defined in crablog.env, an example configuration is provided.
When not using Docker you may have to add crablog.env to your startup script or define the variables there.

`crablog.env`
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

| Route        | Description                                        |
| ------------ | -------------------------------------------------- |
| `/`          | shows the last 5 posts                             |
| `/id/<id>`   | shows a single post by id                          |
| `/all`       | shows all posts                                    |
| `/submit`    | set your submit token and create posts             |
| `/edit/<id>` | edit, delete or hide posts                         |
| `/about`     | information about this blog, social media accounts |
  
**API Routes**

| Route            | Description               |
| ---------------- | ------------------------- |
| `api/blog/posts` | returns all posts as json |
