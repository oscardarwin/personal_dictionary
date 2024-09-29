# Personal Dictionary

## Steps:

- Terminal make new database
- Set up database in code with hardcoded categories.

## Installation:

on first install, run:

```
sudo docker pull vaticle/typedb:latest
sudo docker volume create typedb-data
sudo docker create --name typedb -v typedb-data:/opt/typedb-all-linux-x86_64/server/data -p 1729:1729 --platform linux/amd64 vaticle/typedb:latest
```

## Development
Start and Close typedb core server:
```sudo docker start typedb```
```sudo docker stop typedb```

## TODO:

Run docker in rootless mode and move setup into flake.nix




