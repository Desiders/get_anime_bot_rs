set dotenv-load

host := `uname -a`

# Run docker compose with dev profile
run-docker:
    docker compose --profile dev up --build --force-recreate

# Run docker compose with prod profile
run-docker-prod:
    docker compose --profile prod up --build --force-recreate

# Down docker compose
down-docker:
    docker compose down

# Stop docker compose
stop-docker:
    docker compose stop

# Build docker compose with dev profile
build-docker:
    docker compose --profile dev build

# Build docker compose with prod profile
build-docker-prod:
    docker compose --profile prod build

# Update dependencies
update:
    cargo update

# Clippy
clippy:
    cargo clippy --all --all-features -- -W clippy::pedantic

# Format
format:
    cargo fmt --all -- --check
