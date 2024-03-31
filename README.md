<h1 align="center">
    Wolf Quake :video_game:
</h1>

<p align="center">
    Quake 3 Log Parser
</p>

<div align="center">
    <a href="https://opensource.org/licenses/MIT">
        <img src="https://img.shields.io/badge/License-MIT-yellow.svg" />
    </a>
    <a href="https://dl.circleci.com/status-badge/redirect/gh/nicolasauler/wolf_quake/tree/main">
        <img src="https://dl.circleci.com/status-badge/img/gh/nicolasauler/wolf_quake/tree/main.svg?style=shield&circle-token=CCIPRJ_GZJrRHrrjus3Jhk7LbZQ2s_66ce6241308ef46b8fb0db9f3da02230e410eb78" />
    </a>
    <a href="https://nicolasauler.github.io/wolf_quake/">
        <img src="https://github.com/nicolasauler/wolf_quake/actions/workflows/docs.yml/badge.svg" />
    </a>
    <a href="https://codecov.io/gh/nicolasauler/wolf_quake" >
        <img src="https://codecov.io/gh/nicolasauler/wolf_quake/graph/badge.svg?token=OW4V0Q9Y2F"/>
    </a>
    <a href="https://github.com/nicolasauler/wolf_quake">
        <img src="https://img.shields.io/badge/MSRV-1.74.0-informational" />
    </a>
</div>

## 🏞️ Overview

Wolf Quake is a parser for Quake 3 Arena log files.

You can find a log file: [file](https://gist.github.com/cloudwalk-tests/be1b636e58abff14088c8b5309f575d8)

## :scroll: Documentation

Documentation can be found in:
[DOCUMENTATION](https://nicolasauler.github.io/wolf_quake)

## :computer_mouse: Usage
```shell
Quake 3 log parser

Usage: wolf_quake [OPTIONS] <LOG_FILE>

Arguments:
  <LOG_FILE>  The path to the log file, required

Options:
  -r, --report-type <REPORT_TYPE>      The type of report to generate - Report with player ranking and mean of death ranking - Report with player ranking - Report with mean of death ranking Default: all [default: all] [possible values: all, player-rank, mean-death]
  -f, --report-format <REPORT_FORMAT>  The format of the report to generate - Text table report in console - Html table report Default: text [default: text] [possible values: html, text]
  -o, --output-file <FILE>             The output file to write the report If not provided, the report will be printed to the console
  -h, --help                           Print help (see more with '--help')
  -V, --version                        Print version
```

### Examples
#### Txt report

```console
foo@bar:~$ wolf_quake -f text -o report.txt games.log
```

```shell
╭────────┬──────────────────┬─────────────────┬──────────────────╮
│        │                  │                 │                  │
│        │ Total game kills │ Kill Rank       │  Death Causes    │
│        │                  │ (Player: Score) │  (Cause: Count)  │
│        │                  │                 │                  │
├────────┼──────────────────┼─────────────────┼──────────────────┤
│        │                  │                 │                  │
│ Game 1 │        1         │   Player1: -1   │  TriggerHurt: 1  │
│        │                  │                 │                  │
├────────┼──────────────────┼─────────────────┼──────────────────┤
│        │                  │                 │                  │
│        │                  │   Player2: 1    │ Rocket Splash: 1 │
│ Game 2 │        2         │                 │                  │
│        │                  │   Player1: -1   │ TriggerHurt: 1   │
│        │                  │                 │                  │
╰────────┴──────────────────┴─────────────────┴──────────────────╯
```

#### Html report

```console
foo@bar:~$ wolf_quake -f html -o report.html games.log
```

Not really formatted how it should be due to rendering in markdown, but you can open
[report.html](./examples/qgames_report.html) to see the actual result in your browser.

<table>
<thead>
<tr>
<th>
<div>
<p>

</p>
<p>

</p>
<p>

</p>
</div>
</th>
<th>
<div>
<p>

</p>
<p>
Total game kills
</p>
<p>

</p>
</div>
</th>
<th>
<div>
<p>

</p>
<p>
Kill Rank
</p>
<p>
(Player: Score)
</p>
<p>

</p>
</div>
</th>
<th>
<div>
<p>

</p>
<p>
Death Causes
</p>
<p>
(Cause: Count)
</p>
<p>

</p>
</div>
</th>
</tr>
</thead>
<tbody>
<tr>
<td>
<div>
<p>
Game 1
</p>
</div>
</td>
<td>
<div>
<p>
1
</p>
</div>
</td>
<td>
<div>
<p>

</p>
<p>
Player1: -1
</p>
<p>

</p>
</div>
</td>
<td>
<div>
<p>

</p>
<p>
TriggerHurt: 1
</p>
<p>

</p>
</div>
</td>
</tr>
<tr>
<td>
<div>
<p>
Game 2
</p>
</div>
</td>
<td>
<div>
<p>
2
</p>
</div>
</td>
<td>
<div>
<p>

</p>
<p>
Player2: 1
</p>
<p>

</p>
<p>
Player1: -1
</p>
<p>

</p>
</div>
</td>
<td>
<div>
<p>

</p>
<p>
Rocket Splash: 1
</p>
<p>

</p>
<p>
TriggerHurt: 1
</p>
<p>

</p>
</div>
</td>
</tr>
</tbody>
</table>

## 🦺 Security and 💻 Software

Wolf Quake is written in Rust :crab: and uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.
It also heavily applies clippy lints to ensure the code is idiomatic and follows best practices.

Current status:
- [x] Enviroment setup: CI, local and github
- [x] Happy path log parsing and tests
- [x] Bug handling in original log file
- [x] CLI and tests
- [x] Documentation
- [ ] Better docs.rs reference documentation

For testing, Wolf Quake uses the [proptest](https://docs.rs/proptest/latest/proptest/) crate, which is kind of a more purpose-oriented fuzzy testing tool.

## ⚗️ Contributing
### Install pre-commit
- Depends on Python installed (if not, add the git hook manually)

```shell
curl -LO https://github.com/pre-commit/pre-commit/releases/download/v3.7.0/pre-commit-3.7.0.pyz
python pre-commit-3.7.0.pyz install
```
