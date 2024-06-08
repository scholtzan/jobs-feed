---
sidebar_position: 2
---

# Architecture

Jobs Feed consist of three main components:

* [**Server**](server.md): The server component runs the main logic of extracting job information from source pages, processing user interactions coming from the client, and interacting with the database.
* [**Client**](client.md): The client provides the web-based interface that users interact with.
* [**Database**](database.md): A PostgreSQL database is used to store all information around sources, postings, and configured filters.
