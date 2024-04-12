# Build the frontend
FROM node as build_client

ENV ENVIRONMENT=release
WORKDIR /usr/src/app

COPY ./client ./client
COPY ./server ./server
COPY Makefile Makefile

RUN make build_client


# Build the server
FROM rust as server_build

ENV ENVIRONMENT=release
WORKDIR /usr/src/app

# Build dependencies
COPY ./migration ./migration
COPY Makefile Makefile
COPY server/Cargo.toml ./server/Cargo.toml
COPY server/Cargo.lock ./server/Cargo.lock
RUN mkdir ./server/src && echo 'fn main() {}' > ./server/src/main.rs
RUN cd ./server && cargo build

# Replace with real src
RUN rm -rf ./server/src
COPY ./server/src ./server/src
# Break the Cargo cache
RUN touch ./server/src/main.rs
# Build the project
RUN make build_server

COPY --from=build_client /usr/src/app/server/dist ./server/dist

RUN printf '[default] \n\
address = "0.0.0.0" \n\
port = 3000 \n\
dist_dir = "dist/" \n\
[release.database]  \n\
url = "postgres://postgres:postgres@postgres:5432/jobs_feed_release" \n\
[debug.database]  \n\
url = "postgres://postgres:postgres@postgres:5432/jobs_feed_debug"' > ./server/Rocket.toml

EXPOSE 3000

CMD make server -e ENVIRONMENT=${ENVIRONMENT}
