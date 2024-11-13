# ToDo App API

## Build from Source

### Prerequisits

To build and run the API:

- Rust Compiler
- Cargo (package manager and build tool for Rust)
- Docker

To run the test suite:

- npm
- node

### Build and run the API

1. Clone the repository.

```bash
git clone git@github.com:peshala-prabhapoorna/todo_app_api.git
```

2. Configure env file.

Copy the given `.env.example` file and rename it to `.env`. The values provided
in the env file work as they are. If they are changed make sure to make the
relevant changes in the `docker-compse.yml` file and the env file of the test
suite as well.

```bash
cp .env.example .env
```

3. Spin up the Postgresql database with docker.

```bash
docker compose up
```

4. Build and run the API.

```bash
cargo run
```

### Run the test suite

1. Configure env file.

Copy the given `.env.example` file and rename it to `.env`. The values provided
in the env file work as they are. If they are changed make sure to make the
relevant changes in the `docker-compse.yml` file and the env file of at the
root of the repository as well.

```bash
cp backend-tests/.env-example backend-tests/.env
```

2. Install NPM dependencies.

```bash
npm install
```

3. Build and run the API

4. Run the test suite.

```bash
npm test
```

Happy Coding!
