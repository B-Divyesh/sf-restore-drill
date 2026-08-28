# Restore Drill sample

`sample-backup.sql` contains three harmless orders for the isolated
`restore-drill demo` command. Matching bundled copies live in the crate so the
published binary has the same sample. The command copies these files to a fresh system
temporary directory, runs a normal Docker drill there, and prints that directory
so you can inspect the signed report. It never reads or changes a caller's data.
