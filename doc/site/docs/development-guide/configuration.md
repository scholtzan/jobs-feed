---
sidebar_position: 1
---

# Development Environment

## Requirements

Install the following requirements:
* [Rust](https://www.rust-lang.org/tools/install)
* [Node](https://nodejs.org/en/download)
* [PostgreSQL](https://www.postgresql.org/download/)
* [SeaORM](https://www.sea-ql.org/SeaORM/) (optional, only needed when making changes to the database schema)

## Compiling and Running Jobs Feed

1. After installing the requirement, clone the repository:

```
git clone https://github.com/scholtzan/jobs-feed
cd jobs-feed
```

2. To build both server and client, run:

```
make build
```

3. Jobs Feed stores all its data in a Postgres database, which needs to be created via:

```
psql -h 127.0.0.1 -p 5432 < migration/create_database.sql
```

By default, two databases are created: `jobs_feed_release` (production database) and `jobs_feed_debug` (used for development)


4. To run Jobs Feed, execute:

```
make server
```

Jobs Feed will be running at: http://127.0.0.1:3000

## Development

Instead of only running the server, during development, it is more convenient to run:

```
make run -j 2
```

This will run both the server and automatically rebuild the client whenever changes to its source code are made.

Use the `ENVIRONMENT` variable to switch between `release` and `debug`: 
```
make run -j 2 -e ENVIRONMENT=release
```
These environments serve data from different databases.

### Database

To make changes to the database schema, install [SeaORM](https://www.sea-ql.org/SeaORM/): 

```
cargo install sea-orm-cli
```

To update database schemas, run: 

:::warning

This will delete any existing data in the selected database!

:::

```
DATABASE_URL=postgres://postgres:postgres@localhost:5432/jobs_feed_debug sea-orm-cli migrate refresh
```

To create Rust entities and models based on the updated database schema, run:

```
sea-orm-cli generate entity \
-u postgres://postgres:postgres@localhost:5432/jobs_feed_debug \
-o server/src/entities \
--with-serde both \
--serde-skip-deserializing-primary-key
```

### Formatting

To apply formatting and linting to both `client` and `server` code, run:

```
make format
```
