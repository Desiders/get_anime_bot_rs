<div align="center">

<h1><code>get_anime_bot_rs</code></h1>

<h3>
A telegram bot written to easily get anime images and GIF files of different genres
</h3>

</div>

## Installation

- Install [Docker](https://docs.docker.com/get-docker/) and [Docker Compose](https://docs.docker.com/compose/install/)
- Clone this repository `git clone https://github.com/Desiders/get_anime_bot_rs.git`
- Copy `.env.example` to `.env` and fill it with your data
- Run `docker compose --profile dev up` to start the project in development mode or `docker compose --profile prod up` in production mode.
<br>
You can also use `just` to run the project with `just run-docker` or `just run-docker-prod` commands


## Migrations

To start the migrations, you need to launch the application, install [`sqlx-cli`](https://crates.io/crates/sqlx-cli) or something else for migration purposes, but here we use `sqlx-cli`.
```bash
$ cargo install sqlx-cli --no-default-features --features rustls,postgres
```
Migrations are places in `./src/infrastructure/database/migrations`, so check migrations list and progress:
```bash
$ sqlx migrate info --source ./src/infrastructure/database/migrations --database-url postgres://{user}:{password}@{host}:{port}/{db}
```
Try to run a migration with `dry-run` parameter:
```bash
$ sqlx migrate run --source ./src/infrastructure/database/migrations --database-url postgres://{user}:{password}@{host}:{port}/{db} --dry-run
```
Run a migration:
```bash
$ sqlx migrate run --source ./src/infrastructure/database/migrations --database-url postgres://{user}:{password}@{host}:{port}/{db}
```
Example:
```bash
$ sqlx migrate run --source ./src/infrastructure/database/migrations --database-url postgres://admin:secretpass@127.0.0.1:5432/get_anime_bot
```

For more info, check `README.md` file of [`sqlx-cli`](https://crates.io/crates/sqlx-cli) crate and [`docker-compose`](https://docs.docker.com/compose/compose-file/compose-file-v3/) file docs.

