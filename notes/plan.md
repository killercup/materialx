# Implementation plan

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
>
> I think maybe the steps would go something like
>
> 1. make a crate that can generate an AST for basic MaterialX
> 2. create an asset importer that can import MaterialX as `StandardMaterial`
> 3. upgrade that importer to import a `MaterialXMaterial` that dynamically generates WGSL. look at the `extended_material` example for an example of how to call the guts of the PBR shader.
> 4. start adding more nodes, starting with basic ones like `min` and `max`. each node should generate a fragment of WGSL, much like Hanabi does
>
> after (4) is working, then we can discuss how to do
>
> 5. enable ubershader patterns with custom graph inputs. I'm not quite sure how this should work -- I'm sure it's doable, but we just need to figure it out
