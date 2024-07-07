# Is MaterialX self-described?

It seems that many of the types from the MaterialX spec are described
in `mtlx` files in the `MaterialX` repo
using `nodedef` and other elements.
For example, [this](https://github.com/AcademySoftwareFoundation/MaterialX/blob/v1.39.0/libraries/bxdf/standard_surface.mtlx)
is the definition of the `standard_surface` node.

In theory we could parse these files to get the types.
In practice, we chose to not go that route just yet
to keep this project simple while we focus on the core functionality.

## Discussion notes

@killercup on [discord](https://discord.com/channels/691052431525675048/743663924229963868/1255791894680698881):

> one thing that intrigues me is not hard-coding standard_surface and friends at all and just parsing the nodedefs from [`standard_surface.mtlx`](https://github.com/AcademySoftwareFoundation/MaterialX/blob/v1.39.0/libraries/bxdf/standard_surface.mtlx)
>
> is that what you meant above? so standard_surface is just one of the possible ubershaders we generate?

@pcwalton:

> yeah, so that's the definition of `<standard_surface>`, but if we were to implement that directly I'm pretty sure it would be slow
>
> because mixing BSDFs like that is expensive
>
> instead I think we should have our own custom, but interface-compatible, definition of `<standard_surface>` that's implemented directly in WGSL
>
> well, I guess a "sufficiently smart compiler" could implement BSDF mixing efficiently, but I'm inclined to punt on that
>
> (the problem is that conceptually every BSDF needs to iterate over all the lights, so every BSDF you add causes another trip through the list of lights, which is slow. to efficiently implement it you want to gather up all the lights and iterate through the list of BSDFs once. this requires some annoying loop interchange-ish optimization that seems like it'd be overengineered right now when there are so many other benefits of MaterialX to be had)
>
> for a first cut I think it'd be reasonable to only support `<standard_surface>`, and all the customization would be on the inputs to that node graph. this is the common case anyways
