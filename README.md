# jobs-feed <img src="https://github.com/scholtzan/jobs-feed/blob/main/client/static/favicon.svg" width=26>

> This project is a work in progress

Jobs Feed streamlines the job search process by tracking job postings from various career pages and matching users with roles that align with their skills and preferences.

<img src="https://github.com/scholtzan/jobs-feed/blob/main/doc/site/static/img/main-screen.png" width="1000">

## Features

* View, navigate, and bookmark job postings

<img src="https://github.com/scholtzan/jobs-feed/blob/main/doc/site/static/img/main-screen.png" width="800">

* Add career pages or other sources to extract job postings from
  * Use CSS selectors to specify pagination links and HTML elements that contain job postings

<img src="https://github.com/scholtzan/jobs-feed/blob/main/doc/site/static/img/new-source.png" width="800">

* Configure filters and criterias based on which job postings are selected

<img src="https://github.com/scholtzan/jobs-feed/blob/main/doc/site/static/img/new-filter.png" width="800">
  
* Suggestions of similar companies that might offer relevant roles

<img src="https://github.com/scholtzan/jobs-feed/blob/main/doc/site/static/img/similar-companies.png" width="400">
 
## Installation

### Docker

* Build the docker image: `docker-compose up jobs_feed`
* Run the docker container: `docker-compose run jobs_feed`
* Jobs Feed will be running here: http://127.0.0.1:3000

### From Source

Install the following requirements:
* [Rust](https://www.rust-lang.org/tools/install)
* [Node](https://nodejs.org/en/download)
* [PostgreSQL](https://www.postgresql.org/download/)

* To build the project run: `make build`
* Create the databases: `psql -h 127.0.0.1 -p 5432 < migration/create_database.sql` or create the databases via the `psql` CLI tooling
  * By default two databases are created: `jobs_feed_release` (production database) and `jobs_feed_debug` (used for development)
* To run Jobs Feed execute: `make server`
* Jobs Feed will be running here: http://127.0.0.1:3000

### Configuration

Jobs Feed requires an OpenAI API key. If you don't have an OpenAI API key, [follow these instructions](https://openai.com/blog/openai-api) to obtain one.

The API key needs to be configured in the "Preferences" once Jobs Feed is runnning. 

## Development

Follow the [Installation From Source](#from-source) steps.

* Run `make run -j 2`
  * This will run both the server, and re-built the client whenever changes to its source code are made
  * Use the `ENVIRONMENT` variable to switch between `release` and `debug`: `make run -j 2 -e ENVIRONMENT=release`
    * Both environments serve data from different databases
* Run `make format` to apply formatting and linting to the source code

To make changes to the database schema install [SeaORM](https://www.sea-ql.org/SeaORM/): `cargo install sea-orm-cli`

* To update database schemas, run: `DATABASE_URL=postgres://postgres:postgres@localhost:5432/jobs_feed_debug sea-orm-cli migrate refresh`
  * ⚠️ This will delete any existing data
* To create Rust entities and models based on the updated database schema, run: `sea-orm-cli generate entity -u postgres://postgres:postgres@localhost:5432/jobs_feed_debug -o server/src/entities --with-serde both --serde-skip-deserializing-primary-key`

## Roadmap

* Find pagination links on source pages automatically
* Pulling job postings on a schedule
* Grouping sources
* Translations
* Different layouts
* Send alerts or email summaries
* User management

## Contributing

Contributions are welcome. If you'd like to help, please file an issue or open a pull request.
