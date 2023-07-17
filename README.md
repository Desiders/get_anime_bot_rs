# get_anime_bot_rs
It's a telegram bot for convenient getting anime GIFs or images by genres 

# Migrations

### Docker
To start the migrations, you need to launch the application, uncomment two lines in `docker.compose.yaml` so that you can connect to the database in the docker container:
```yaml
extra_hosts:
    - "host.docker.internal:host-gateway"
```
Further, you need to install [`sqlx-cli`](https://crates.io/crates/sqlx-cli) or something else for migration purposes, but here we use it.
```bash
$ cargo install sqlx-cli --no-default-features --features rustls,postgres
```
Migrations are places in `./src/infrastructure/database/migrations`, so check migrations list and progress:
```bash
$ sqlx migrate info --source ./src/infrastructure/database/migrations --database-url postgres://{user}:{password}@{host}:{port}/{db}
20230620173320/pending create-tables
```
Try to run a migration with `dry-run` parameter:
```bash
$ sqlx migrate run --source ./src/infrastructure/database/migrations --database-url postgres://{user}:{password}@{host}:{port}/{db} --dry-run
Can apply 20230620173320/migrate create-tables (0ns)
```
Run a migration:
```bash
$ sqlx migrate run --source ./src/infrastructure/database/migrations --database-url postgres://{user}:{password}@{host}:{port}/{db}
Applied 20230620173320/migrate create-tables (27.475867ms)
```
Comment out the lines in `docker-compose.yaml` back:
```yaml
# extra_hosts:
#     - "host.docker.internal:host-gateway"
```

For more info, check `README.md` file of [`sqlx-cli`](https://crates.io/crates/sqlx-cli) crate and [`docker-compose`](https://docs.docker.com/compose/compose-file/compose-file-v3/) file docs.

