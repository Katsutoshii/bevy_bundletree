# `bevy_bundletree`

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/Katsutoshii/bevy_bundletree#license)
[![Crates.io](https://img.shields.io/crates/v/bevy_bundletree.svg)](https://crates.io/crates/bevy_bundletree)
[![Docs](https://docs.rs/bevy_bundletree/badge.svg)](https://docs.rs/bevy_bundletree/latest/bevy_bundletree/)

Spawn trees of bundles in Bevy to make UI Code more ergonomic.

## Usage

Define an enum to represent all possible bundles in your tree and derive `IntoBundleTree` and `BundleEnum`.

```rust
use bevy::prelude::*;
use bevy_bundletree::*;

#[derive(IntoBundleTree, BundleEnum)]
enum UiNode {
    Node(NodeBundle),
    Text(TextBundle),
    Button(ButtonBundle),
}
fn setup(mut commands: Commands) {
    let tree: BundleTree<UiNode> = NodeBundle::default().with_children([
        TextBundle::default().into_tree(),
        ButtonBundle::default().into_tree()]);
    commands.spawn_tree(tree);
}
```
