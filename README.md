# Tembo CLI

Tembo CLI allows users to experience [Tembo](https://tembo.io) locally, as well as, 
manage and deploy to Tembo Cloud. It abstracts away complexities of configuring, 
managing, and running Postgres in a local environment. 

# Local Testing

Clone this repo and run:

`cargo install --path .`

If the install path is in your shell path, you can then run `tembo help` and other `tembo` commands.

# Commands

## `tembo init`

The `init` command initializes your environment with following files:

* `tembo.toml` configuration file
* `migrations` directory for sql migrations
* `~/.tembo/context` file with various contexts user can connect to

For more information: `tembo init --help`

## `tembo context list/set`

tembo context works like how [kubectl context](https://www.notion.so/abee0b15119343e4947692feb740e892?pvs=21) works. User can set context for local docker environment or tembo cloud (dev/qa/prod) with org_id. When they run any of the other commands it will run in the context selected. Default context will be local.

## `tembo apply`

Validates Tembo.toml (same as `tembo validate`) and applies the changes to the context selected.

* applies changes and runs migration for all dbs
    * **local docker:** wraps docker build/run + sqlx migration
    * **tembo-cloud:** calls the api in appropriate environment

## `tembo delete`

- **local docker:** runs `docker stop & rm` command
- **tembo-cloud:** calls delete tembo api endpoint

## Generating Rust Client from API

[OpenAPI Generator](https://openapi-generator.tech/) tool is used to generate Rust Client.

Install OpenAPI Generator if not already by following steps [here](https://openapi-generator.tech/docs/installation)

### Control plane API client

Go to `src/temboclient` directory in your terminal.

Delete the contents of the directory first and then run following command to re-generate the rust client code for the API.

```bash
openapi-generator generate -i https://api.tembo.io/api-docs/openapi.json  -g rust -o . --additional-properties=packageName=temboclient
```

# Contributing

Before you start working on something, it's best to check if there is an existing plan 
first. Join our [Slack community](https://join.slack.com/t/trunk-crew/shared_invite/zt-1yiafma92-hFHq2xAN0ukjg_2AsOVvfg) and ask there.

# Semver

Tembo CLI is following [Semantic Versioning 2.0](https://semver.org/).
