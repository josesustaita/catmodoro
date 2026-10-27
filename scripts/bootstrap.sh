#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.."
if ! command -v docker >/dev/null 2>&1; then
  echo "Docker is required. Install Docker Engine or Docker Desktop, then retry." >&2
  exit 1
fi
if ! docker compose version >/dev/null 2>&1 || ! docker info >/dev/null 2>&1; then
  echo "Start Docker and enable the Docker Compose v2 plugin, then retry." >&2
  exit 1
fi
if [[ ${1:-} == doctor ]]; then
  docker compose config --quiet
  echo "Ready. Run ./scripts/bootstrap.sh -d 25 (use an interactive terminal)."
  exit 0
fi
if [[ ${1:-} == test ]]; then
  docker compose build
  docker compose run --rm -T catmodoro --help
  exit 0
fi
if [[ ! -t 0 || ! -t 1 ]]; then
  # Help and version work in CI; the timer itself needs a terminal.
  case ${1:-} in
    --help|-h|--version|-V) exec docker compose run --build --rm -T catmodoro "$@" ;;
    *) echo "Use an interactive terminal, or run ./scripts/bootstrap.sh test." >&2; exit 1 ;;
  esac
fi
exec docker compose run --build --rm catmodoro "$@"
