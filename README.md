# pj

pj is a CLI tool to manage and easily switch between projects.

## Prerequisites

* Currently pj only works for bash and zsh shells.
* cargo (for building the Rust program)

## Installation

* `git clone https://github.com/aimenhamed/pj`
* `./install.sh`

## Usage

* `pj` - initializes config on first run or prompt to switch project
* `pj list` - lists added projects
* `pj add {dir}` - add project
* `pj remove {project_name}` - removes a project

## Information

Upon prompting `pj` you will get a list of added projects which you can then select via index, and it'll `cd` into that directory.

e.g.

```bash
[aimen:~]$ pj
Available projects:
0. pj
1. decorate
2. lss
Please pick a project (index):
0
[aimen:~/repos/pj]$
```
