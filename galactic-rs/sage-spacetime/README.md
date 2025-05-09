```
# https://spacetimedb.com/install
spacetime init --lang rust server
```

## Quickstart

```
spacetime start
cargo run -p stdb-relay
cargo run -p stdb-client
```

## Development

```
just server-start
# spacetime start

just server-publish
# spacetime delete sage-stdb
# spacetime publish --server local --project-path server sage-stdb
```

### Client/Relay

```
just generate
# spacetime generate --lang rust --out-dir client/src/module_bindings --project-path server
# spacetime generate --lang rust --out-dir relay/src/module_bindings --project-path server
```

## Standalone

https://spacetimedb.com/docs/deploying/spacetimedb-standalone

## Examples

* https://github.com/ClockworkLabs/Blackholio
* https://github.com/GalaxyCr8r/solarance-beginnings/
* https://github.com/Moreno-Gentili/space-vs-time
* https://github.com/SeloSlav/vibe-coding-starter-pack-2d-multiplayer-survival
* https://docs.rs/rstar/latest/rstar/primitives/struct.GeomWithData.html

* https://github.com/GalaxyCr8r/solarance-beginnings/pull/30/files#diff-7c3e8b7ae91394e8c07c84e845202af8ca33f4563bffd075cb1ef7b6db86b864R94
* https://github.com/GalaxyCr8r/solarance-beginnings/pull/30/files#diff-6a7ec327336edab1ae223d43992fdda7a8c6b5cad3cce55a8db0926c00448026
