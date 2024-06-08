---
sidebar_position: 3
---

# Database

Jobs Feed stores all data in a PostgreSQL database. To facilitate the interaction between the server and the database, it uses [SeaORM](https://www.sea-ql.org/SeaORM/docs/introduction/orm/), which helps create tables and generate Rust structs representing the entities stored in the database.

The table schemas and migration logic are located in the `migration/` directory. Each source file represents a separate table. Schema changes and new tables need to be defined in this directory and then applied to the database using the `sea-orm-cli`:

```
DATABASE_URL=postgres://postgres:postgres@localhost:5432/jobs_feed_debug sea-orm-cli migrate refresh
```

This will delete any previously stored data.

To create Rust entities and models based on the updated database schema, run:

```
sea-orm-cli generate entity \
-u postgres://postgres:postgres@localhost:5432/jobs_feed_debug \
-o server/src/entities \
--with-serde both \
--serde-skip-deserializing-primary-key
```

The generated Rust structs will be placed in the `server/src/entities/` directory and can be referenced by the server code.
