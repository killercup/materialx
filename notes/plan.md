# Implementation plan

(Quote edited for Markdown formatting.)

On [discord](https://discord.com/channels/691052431525675048/743663924229963868/1254550759937409134),
[@pcwalton](https://github.com/pcwalton) recommended the following steps
to implement [MaterialX](https://materialx.org) support in [Bevy](https://bevyengine.org/)
(formatting edited):

> Great! I think a good first step would be to parse enough MaterialX to duplicate `StandardMaterial`. Here's an example of a material like that <https://github.com/AcademySoftwareFoundation/MaterialX/blob/main/resources/Materials/Examples/StandardSurface/standard_surface_jade.mtlx>
>
> The spec is here <https://materialx.org/Specification.html>
>
> You'll want to look through the Core Specification and the Physically Based Shading Nodes spec
>
> Definitely don't try to parse all of it at once. It's a huge spec and _nobody_ implements all of it (except maybe Pixar internally)

## Steps

@pcwalton on [discord](https://discord.com/channels/691052431525675048/743663924229963868/1254551846325063701):

> 1. make a crate that can generate an AST for basic MaterialX
> 2. create an asset importer that can import MaterialX as `StandardMaterial`
> 3. upgrade that importer to import a `MaterialXMaterial` that dynamically generates WGSL. look at the `extended_material` example for an example of how to call the guts of the PBR shader.
> 4. start adding more nodes, starting with basic ones like `min` and `max`. each node should generate a fragment of WGSL, much like Hanabi does
>
> after (4) is working, then we can discuss how to do
>
> 5. enable ubershader patterns with custom graph inputs. I'm not quite sure how this should work -- I'm sure it's doable, but we just need to figure it out

## MaterialX integration

@pcwalton on [discord](https://discord.com/channels/691052431525675048/743663924229963868/1255707273028960256):

> - You can load MaterialX using the sub-resource syntax of the asset loader: `asset_loader.load("my_materialx_file.mtlx#my_material")`
> - The `#my_material` may refer to either a node with type `material` or a `<nodegraph>` with an output type `material`:
>
>   - If `#my_material` refers to a `material` node, then the resulting asset is a `MaterialXMaterial`, which implements `Material` as usual.
>     - For now, all such materials will be considered separate and will not be batched together. But read on for a better way:
>   - If `#my_material` refers to a `nodegraph` node, then the asset is a `MaterialXNodeGraph`, which is not itself a `Material` but can become one by specifying values for the `<input>`s:
>
>     ```rust
>     let mut my_material_node_graph: Handle<MaterialXNodeGraph> = asset_server.load("my_materialx_file.mtl#my_material");
>     let mut my_material: Handle<MaterialXMaterial> = my_material_node_graph.build((
>         ("base_color", GOLDENROD),
>         ("normal", my_normal_map),
>     ));
>     ```

Regarding different material types
([discord](https://discord.com/channels/691052431525675048/743663924229963868/1255707907459387562)):

> I still feel like StandardMaterial can be useful for convenience' sake
> but I anticipate MaterialXMaterial will be common
>
> My current thinking is that some features, like triplanar mapping and detail maps, will only be supported in MaterialX

## Optimizing how generated materials performance

@pcwalton on [discord](https://discord.com/channels/691052431525675048/743663924229963868/1255707273028960256):

> - All `MaterialXMaterial`s built from the same `MaterialXNodeGraph` use the same WGSL shader, allowing for shader reuse.
>   - Apps can use a `MaterialXNodeGraph` to produce their own custom ubershaders.
>     - Question: Should there be a way to define an ubershader and supply variants of it within MaterialX, without having to use the Bevy editor?

> I think the answer is: if multiple materials reference the same type of surfaceshader node, then they will be batched

> One possibility is to create a single shader for each node type referenced by a `surfaceshader` directly targeted by a material imported, and to reuse it among all materials that reference that `surfaceshader`. For example `<standard_surface>` would get a single shader.

> In any case, the way you're intended to make an ubershader is through `<nodegraph>` elements.
> Along with the corresponding `<nodedef>` this allows you to define arbitrary inputs, and to use substitutions.
> so you could have a `<nodegraph>` that takes the filename of a texture as an input, and then instantiate that repeatedly with different textures.
> This could turn into a batchable shader on the Bevy side.
> See [spec](https://github.com/AcademySoftwareFoundation/MaterialX/blob/main/documents/Specification/MaterialX.Specification.md#custom-node-definition-using-node-graphs) -- a Bevy ubershader would be a "functional nodegraph"

### Using variants

@pcwalton on [discord](https://discord.com/channels/691052431525675048/743663924229963868/1255780213745258579):

> [Material variants](https://github.com/AcademySoftwareFoundation/MaterialX/blob/main/documents/Specification/MaterialX.Specification.md#material-variants)
> -- this looks like what we want.
> So to define a bunch of materials that all use the same ubershader, you would define the ubershader as a `<nodegraph>`, and then write `<variant>`s for each material.
> The `<variant>` supplies concrete values to the node's inputs.
> Note that `<variant>` elements have to be fully resolved values; they can't be nodes. This is exactly what we need to be able to use ubershaders.
>
> As an alternative, you could define your ubershader as a `<nodegraph>`, and then specialize that shader in BSN, not in MaterialX.
> Both ways should be supported.

> OK, so to update the strawperson proposal:
>
> As an alternative to `<nodegraph>`, you can load a `<variant>` with the syntax
>
> ```rust
> let my_material: Handle<MaterialXMaterial> = asset_server.load("my_materialx_file.mtlx#my_variantset/my_variant");
> ```
>
> Assuming MaterialX that looks like the following:
>
> ```xml
> <nodedef name="my_ubershader">...</nodedef>
> <nodegraph name="my_ubershader">...</nodegraph>
> <variantset name="my_variantset" nodedef="my_ubershader">
>     <variant name="my_variant">...</variant>
> </variantset>
> ```
>
> This will instantiate the material contained within the `<nodegraph>` with the appropriate values in the `<variant>`. All variants attached to `my_ubershader` are guaranteed to use the same WGSL shader, and are therefore batchable.
>
> Note that the `variantset` _must_ contain a `nodedef` attribute for Bevy to know how to instantiate it.
>
> One cool thing is that `<nodedef>` standardizes a bunch of UI hints, such as min/max values and folder groupings, so the Bevy editor will automatically be able to create an artist-friendly UI for your ubershaders.
>
> Another cool thing: MaterialX actually has a syntax for specifying custom nodes implemented with shader code. This means that you wouldn't need to write any Rust to write your own nodes, just WGSL!
>
> One downside is that random `.mtlx` files you find on the Internet might not be well-optimized for Bevy, in that they may not try to group into ubershaders. I don't see a great solution for this, other than to maybe detect common patterns and optimize them. Not sure it's worth it.
