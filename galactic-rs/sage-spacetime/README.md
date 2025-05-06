```
# https://spacetimedb.com/install
spacetime init --lang rust server
```

## Quickstart

```
spacetime start
cargo run -p relay
cargo run -p client
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
