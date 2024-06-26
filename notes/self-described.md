# Is MaterialX self-described?

It seems that many of the types from the MaterialX spec are described
in `mtlx` files in the `MaterialX` repo
using `nodedef` and other elements.
For example, [this](https://github.com/AcademySoftwareFoundation/MaterialX/blob/7f41da2bb5c950be6b9ee84070994c9d8fc32685/libraries/bxdf/standard_surface.mtlx)
is the definition of the `standard_surface` node.

In theory we could parse these files to get the types.
In practice, we chose to not go that route just yet
to keep this project simple while we focus on the core functionality.
