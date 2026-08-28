#!/bin/sh
set -eu

: "${RESTORE_DRILL_CONFIG:?Set RESTORE_DRILL_CONFIG to an absolute config path}"
: "${RESTORE_DRILL_LOG_DIR:?Set RESTORE_DRILL_LOG_DIR to a private output directory}"

umask 077
mkdir -p "$RESTORE_DRILL_LOG_DIR"
stamp=$(date -u +%Y-%m-%dT%H%M%SZ)

# The command's exit code is preserved. Its signed report is written to the
# report_dir in RESTORE_DRILL_CONFIG; this JSON copy is useful for job logs.
exec restore-drill run --config "$RESTORE_DRILL_CONFIG" --json > "$RESTORE_DRILL_LOG_DIR/$stamp.json"
