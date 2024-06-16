---
sidebar_position: 2
---

# Quick Start

## Installation

Jobs Feed can be quickly set up using [Docker](https://www.docker.com/). 

1. Download the source code from [Github](https://github.com/scholtzan/jobs-feed)
2. Build the Docker image: `docker-compose up jobs_feed`
3. Open http://127.0.0.1:3000

## Getting Started with Jobs Feed

Jobs Feed requires an OpenAI API key, which can be found on the [OpenAI API keys page](https://platform.openai.com/api-keys).
This API key will be used to extract relevant job postings from career pages.

Copy the API key and go to the [preferences page in Jobs Feed](http://127.0.0.1:3000/preferences/). Insert the API key and select the preferred [OpenAI model](https://platform.openai.com/docs/models).

Jobs Feed is now ready to be used. Add some filters to narrow down relevant job postings and a add new sources. 
