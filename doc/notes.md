# Development notes

## Database creation

* sea-orm
* `sea-orm-cli migrate init`
* using `psql` create a new `jobs_feed` database: `create database jobs_feed;`
* update database schemas: `DATABASE_URL=postgres://postgres:postgres@localhost:5432/jobs_feed sea-orm-cli migrate refresh`
* create entities/models: `sea-orm-cli generate entity -u postgres://postgres:postgres@localhost:5432/jobs_feed -o server/src/entities --with-serde both --serde-skip-deserializing-primary-key`
* todo: wrap all this into a setup script