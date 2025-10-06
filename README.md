# `bevy_bundletree`

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/Katsutoshii/bevy_bundletree#license)
[![Crates.io](https://img.shields.io/crates/v/bevy_bundletree.svg)](https://crates.io/crates/bevy_bundletree)
[![Docs](https://docs.rs/bevy_bundletree/badge.svg)](https://docs.rs/bevy_bundletree/latest/bevy_bundletree/)

Spawn trees of bundles in Bevy to make UI Code more ergonomic.

The current version heavily based on https://github.com/Leafwing-Studios/i-cant-believe-its-not-bsn.

## Usage

```rust
use bevy_ecs::prelude::*;
use bevy_bundletree::ChildBundle;

#[derive(Component)]
struct A;

#[derive(Component)]
struct B(u8);

fn spawn_hierarchy(mut commands: Commands) {
  commands.spawn(
   (A, // Parent
    ChildBundle( // This component is removed on spawn
      (A, B(3)) // Child
    )
  ));
}
```


## Bevy support table

| bevy | bevy_bundletree |
| ---- | --------------- |
| 0.16 | 0.6.0           |
| 0.15 | 0.3.0-0.5.0     |
