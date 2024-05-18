/// Crate to allow spawning enum bundles as trees.
use bevy::{
    ecs::{
        bundle::Bundle,
        entity::Entity,
        system::{Commands, EntityCommands},
    },
    hierarchy::BuildChildren,
};

/// Spawn a tree of bundles.
/// We don't use the Bundle trait directly because that trait doesn't support enums.
///
/// ```
/// use bevy::prelude::*;
/// use bevy_bundletree::*;
///
/// #[derive(IntoBundleTree, BundleEnum)]
/// enum UiNode {
///     Node(NodeBundle),
///     Text(TextBundle),
///     Button(ButtonBundle),
/// }
/// fn setup(mut commands: Commands) {
///     let tree: BundleTree<UiNode> = NodeBundle::default().with_children([
///         TextBundle::default().into_tree(),
///         ButtonBundle::default().into_tree()]);
///     commands.spawn_tree(tree);
/// }
/// ```
#[derive(Clone)]
pub struct BundleTree<B: BundleEnum> {
    pub bundle: B,
    pub children: Vec<BundleTree<B>>,
}
impl<B: BundleEnum> BundleTree<B> {
    pub fn new(bundle: impl Into<B>) -> Self {
        Self {
            bundle: bundle.into(),
            children: Vec::default(),
        }
    }
    pub fn with_children(mut self, children: impl IntoIterator<Item = BundleTree<B>>) -> Self {
        self.children = children.into_iter().collect();
        self
    }
}

/// Make it easy to convert bundles into their corresponding trees.
pub trait IntoBundleTree<B: BundleEnum>: Bundle + Into<B> {
    fn into_tree(self) -> BundleTree<B> {
        BundleTree::new(self)
    }
    fn with_children(self, children: impl IntoIterator<Item = BundleTree<B>>) -> BundleTree<B> {
        self.into_tree().with_children(children)
    }
}

/// Support calling bundle.into() to get a `BundleTree`.
impl<B: BundleEnum> From<B> for BundleTree<B> {
    fn from(bundle: B) -> Self {
        BundleTree::new(bundle)
    }
}

/// A bundle that can be spawned via commands.
pub trait BundleEnum {
    fn spawn<'c>(self, commands: &'c mut Commands) -> EntityCommands<'c>;
}

/// Trait for using commands to spawn BundleTree<B>.
pub trait BundleTreeSpawner {
    /// Spawns a BundleTree and returns the EntityCommands for the root.
    fn spawn_tree<B: BundleEnum>(&mut self, tree: BundleTree<B>) -> EntityCommands;
}
impl BundleTreeSpawner for Commands<'_, '_> {
    fn spawn_tree<B: BundleEnum>(&mut self, tree: BundleTree<B>) -> EntityCommands {
        let BundleTree { bundle, children } = tree;

        let mut child_ids: Vec<Entity> = Vec::with_capacity(children.len());
        for child in children.into_iter() {
            let entity = self.spawn_tree(child).id();
            child_ids.push(entity);
        }

        let mut e = bundle.spawn(self);
        e.push_children(&child_ids);
        e
    }
}

/// Trait for building a tree from a struct with custom context.
pub trait MakeBundleTree<B: BundleEnum, Context> {
    /// Returns the BundleTree associated with the given type.
    fn tree(self, context: Context) -> BundleTree<B>;
}

// Re-export derive macros.
pub use bevy_bundletree_derive::{BundleEnum, IntoBundleTree};

#[cfg(test)]
mod test {
    use crate::*;
    use bevy::prelude::*;

    // Make sure that all entities in the tree are spawned.
    #[test]
    fn basic_test() {
        #[allow(clippy::large_enum_variant)]
        #[derive(IntoBundleTree, BundleEnum)]
        enum UiNode {
            Node(NodeBundle),
            Text(TextBundle),
            Button(ButtonBundle),
        }

        fn setup(mut commands: Commands) {
            let tree: BundleTree<UiNode> = NodeBundle::default().with_children([
                TextBundle::default().into_tree(),
                ButtonBundle::default().into_tree(),
            ]);
            commands.spawn_tree(tree);
        }

        // Setup app
        let mut app = App::new();
        app.add_systems(Startup, setup);

        // Run systems
        app.update();

        // Check enemy was despawned
        assert!(app.world.entities().total_count() == 3);
    }
}
