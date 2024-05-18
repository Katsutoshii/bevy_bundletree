# `bevy_bundletree`

Spawn trees of bundles in Bevy to make UI Code more ergonomic.

## Usage

Define an enum to represent all possible bundles in your tree and derive `IntoBundleTree` and `BundleEnum`.

```rs
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
