# Restore Drill copy audit — polish 4

Counts use visible alphanumeric words. Hyphenated terms and commands count as
one word. No unit exceeds 22 words. No public copy uses a banned marketing word.

## Landing page

| Words | Copy |
| ---: | --- |
| 2 | Restore Drill |
| 1 | Demo |
| 3 | How it works |
| 1 | Privacy |
| 3 | Postgres restore rehearsal |
| 5 | Prove your Postgres backup restores. |
| 11 | For small self-hosted teams that need recovery proof before an outage. |
| 5 | Try it with sample data |
| 10 | Replays a recorded sample restore and opens its signed report. |
| 6 | Runs with your local Docker engine |
| 7 | Sample files stay in a temporary directory |
| 3 | MIT-licensed command-line tool |
| 4 | Plate 01 / isolated rehearsal |
| 14 | A sealed backup enters an isolated test chamber and emerges as a healthy database |
| 3 | Sample backup in. |
| 3 | Signed report out. |
| 4 | How the command works |
| 6 | Rehearse a restore in four steps. |
| 5 | Create an isolated Docker network. |
| 10 | The command uses a new internal network for the rehearsal. |
| 5 | Restore a shipped sample backup. |
| 14 | The demo uses a SQL file with three sample orders in a temporary directory. |
| 4 | Check the restored data. |
| 12 | The sample confirms that all three orders are present after the restore. |
| 7 | Sign the report and remove test resources. |
| 11 | Keep the report path printed by the command for later inspection. |
| 3 | Start for real |
| 7 | Install one binary on your Docker host. |
| 12 | Read the configuration before you run a drill against your own backup. |
| 5 | Read the source (opens GitHub) |
| 6 | Read the setup guide (opens GitHub) |
| 5 | What this does not do |
| 4 | It rehearses a restore. |
| 5 | It does not restore production. |
| 10 | Use a copied backup and a Docker host you control. |
| 10 | Read the privacy policy for site and command data handling. |
| 3 | Read privacy details |
| 2 | 3 orders |
| 1 | internal |
| 6 | signed report you can check later |
| 2 | restore-drill demo |
| 6 | Recovery proof for self-hosted Postgres teams. |
| 4 | Built by Param Factory |

## Demo first screen

| Words | Copy |
| ---: | --- |
| 7 | Demo — sample data, nothing is saved to your data. |
| 2 | Reset demo |
| 3 | View installation steps |
| 3 | Verified web walkthrough |
| 6 | Replay a sample Postgres restore. |
| 15 | Watch a recorded Docker run restore three sample orders. Then inspect the signed report it produced. |
| 6 | Recorded on a GitHub-hosted Linux runner |
| 2 | Restore transcript |
| 2 | Pause replay |
| 12 | Recorded output from the real demo command. Times and temporary paths vary by computer. |
| 2 | Generated evidence |
| 5 | Inspect the signed sample report. |
| 3 | Download signed report |
| 5 | Open report in this tab |

## README check

README prose was read sentence by sentence. The longest sentence is 20 words.
“Continuous integration” was replaced with “automation.” The report is called
a “signed report file you can check later.” Install steps start with `git clone`
and match the public `#install-from-source` link.

## Terminology

| Concept | Word used |
| --- | --- |
| User-supplied recovery input | backup |
| Browser experience | web walkthrough |
| Executable sample operation | demo command |
| Full recovery check | drill |
| Isolated connectivity | internal Docker network |
| Evidence output | report |
| Scheduled execution | weekly drill |

Flags: none.
