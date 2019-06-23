# Pluginbaby

**Notice: This project is WIP and highly experimental.**

Update git repositories at once.

Heavely inspired by [pack](https://github.com/maralla/pack).

## Motivations

- To learn Rust language
- Want to update my Vim8 pack directories at once

## Usage

### List repositories

List git repositories to stdout.

`$ pluginbaby list`

Retrive current directory(default max depth is 3) and list git repository info.

`$ Pluginbaby list --path=~/.vim/pack --depth=5`

Retrive `~/.vim/pack` directory and list git repository info.

### Dump repositories

Dump git repositories info to `Repofile`.

`$ pluginbaby dump`

Retrive current directory(default max depth is 3) and if directory contains `.git` directory, dump git repository info to `Repofile`.

`$ pluginbaby dump --path=~/.vim/pack --dist=~/`

Dump `~/.vim/pack` directory and save `Repofile` to home directory.

### Restore repositories

Restore(`git clone`) from `Repofile`.

`$ pluginbaby restore --conf=./Repofile --dist=./mypack`

Do `git clone` to `./mypack` directory.

### Update repositories

`$ pluginbaby update --path=~/.vim/pack

Do `git fetch origin/master` and `reset --hard` to `./mypack` directory.
