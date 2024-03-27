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
        <img src="https://img.shields.io/badge/MSRV-1.58.1-informational" />
    </a>
</div>

## 🏞️ Overview

Wolf Quake is a parser for Quake 3 Arena log files.

You can find the log file: [file](https://gist.github.com/cloudwalk-tests/be1b636e58abff14088c8b5309f575d8)

:warning: Wolf Quake is a wip

## :scroll: Documentation

Documentation can be found in:
[DOCUMENTATION](https://nicolasauler.github.io/wolf_quake)

## 🦺 Security and 💻 Software

Wolf Quake is written in Rust :crab: and uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

Current status:
- [x] Enviroment setup: CI, local and github
- [ ] Development

## ⚗️ Contributing
### Install pre-commit
- Depends on Python installed (if not, add the git hook manually)

```shell
curl -LO https://github.com/pre-commit/pre-commit/releases/download/v3.7.0/pre-commit-3.7.0.pyz
python pre-commit-3.7.0.pyz install
```
