#!/usr/bin/env bash

set -Eeuo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"

# Gemeinsame Funktionen und Pfade liegen in @shell/init.sh.
# shellcheck source=/dev/null
source "$ROOT_DIR/@shell/init.sh"

main "$ROOT_DIR" "$@"
