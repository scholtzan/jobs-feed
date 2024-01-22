# Development notes

## Database creation

* sea-orm
* `sea-orm-cli migrate init`
* using `psql` create a new `jobs_feed` database: `create database jobs_feed;`
* update database schemas: `DATABASE_URL=postgres://postgres:postgres@localhost:5432/jobs_feed sea-orm-cli migrate refresh --migration-dir server/`
* create entities/models: `sea-orm-cli generate entity -u postgres://postgres:postgres@localhost:5432/jobs_feed -o server/src/entities --with-serde both --serde-skip-deserializing-primary-key `
* todo: wrap all this into a setup script
  
## High-level UI

* planning elements for MVP
  * sidebar
  * adding sources
  * top bar
  * postings listing
  * postings view

## Rocket setup

* 