# Schedule a weekly drill

Copy `restore-drill-weekly.sh` to `/usr/local/bin/` and make it executable. Add
the line in `restore-drill.crontab` with absolute paths for your host.

The GitHub Actions example expects `ops/restore-drill.toml` and a backup in the
repository workspace. Store only `RESTORE_DRILL_POSTGRES_PASSWORD` as an
encrypted repository secret. Change the paths to match your private repository.

Both examples preserve the command's failing exit code. The cron runner keeps a
JSON job result, while Restore Drill writes the signed report to its configured
report directory. The workflow uploads both files even after failure.
