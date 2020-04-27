[![Docker-build-push](https://github.com/nmrshll/auth-rs-warp/workflows/Docker-build-push/badge.svg)](https://github.com/nmrshll/auth-rs-warp/actions?query=workflow%3ADocker-build-push)

# auth-rs-warp

Authentication / Authorization server example with Rust, Warp, Postgres

## Features

- **Register users with email/password**
- **Get an exsisting user / Check if an email is already taken**
- **Login / Logout with email/password**
- **Access / modify protected resources**: one example included: creating posts that belong to a user

and also:

- **Great performance and minimal footprint** thanks to Rust, Hyper, tokio and futures / async-await
- **Users database** using Postgres

### Limitations

- As of now, this example is usable for writing API routes in Rust (i.e. starting from this code and extending). Usage with an other stack might be possible but wasn't yet taken into consideration for this project.

## Usage

### Prerequisites

- **Docker** daemon running
- **Rust 1.41+** (auto-installs if missing)

### Running the auth server

Three options: On your local machine with Docker, from the Docker image, or using kubernetes

#### On your local machine

Run `make`

This will start docker containers for: the API, the postgres database,

#### From the docker image

Run

```shell
docker run -p 0.0.0.0:8080:8080 -e DATABASE_URL=postgres://user:pass@postgres/db docker.pkg.github.com/nmrshll/auth-rs-warp/api:latest
```

In this configuration you need to provide your own postgres server, and run the migrations onto it manually.

#### With Kubernetes

Kubernetes deployment file are included. They need to be applied with [CUE](https://cuelang.org/). Examples of how do do that are included in the [makefile](./makefile)

### Configuration options

Configuration is applied, from highest to lowest priority, through:

- Environment variables
- Config files (can be `JSON`, `YAML`, `TOML`, `HCL`, `INI`)
- Hardcoded defaults

These options are:

| Option            | ENV_VAR name      | Config name |
| ----------------- | :---------------- | :---------- |
| Database URL      | DATABASE_URL      | \$1600      |
| Postgres user     | POSTGRES_USER     | \$12        |
| Postgres password | POSTGRES_PASSWORD | \$1         |
| Postgres database | POSTGRES_DB       | \$1         |

## Testing

### Automated

Local testing is available by running

```shell
make test
```

### Manual

Test requests are included in the payload (using `curl`)

- **Register a user** using `make users/register`
- **Check if an email is already taken** using `make users/check`
- **Login** using `make users/login`
- **Access a dummy protected route** using `make protected`

## TODO and contributing

Don't hesitate to file a bug, request a feature, or simply comment using issues.

If you want to help here's a few useful tasks I have in mind for this project:

- [ ] Email verification and email invitations
- [ ] CI
- [ ] example deployment instructions
- [ ] Write a tutorial to re-create this repo from scratch
