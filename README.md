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

### Starting the server

Run `make`

## Testing

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
