# Bevy Asset Loading

Bevy comes with an `AssetServer`, which is used to load and manage assets.

## Addressing Assets

Assets are identified by a URI, usually a filesystem path.
It might include a fragment to specify a sub-asset within a file.

## Loading Assets

Loading assets is done asynchronously,
and the systems that trigger loading assets might not be same that use them.
Calling `load` on the same asset twice will only load it once.

Components that deal with asset data accept a `Handle<T>` instead of the asset itself.
They will be updated when the asset is loaded.

Loading assets also emits `EventReader<AssetEvent<T>>` when an asset of the type `T` was loaded.
This event is an enum and only carries what happened to the asset and its ID.
To go from the ID to the asset, use `Res<Assets<T>>.get(id)`,
or use `Res<AssetServer>.get_path(id)` to get the path.

## Custom Loaders

Any type implementing `AssetLoader` can registered as an asset loader in the `App`.

## Pre-processors

Assets can be pre-processed before they are loaded.
This way an asset could be translated from one format into another one
before the game accesses it.

TODO: When does it take place?
