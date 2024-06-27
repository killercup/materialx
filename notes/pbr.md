# What Properties Materials Use

[MaterialX] can describe complete rendering shaders using node graphs
but it also has [PBR Spec] which define "shader-semantic nodes".
[Bevy]'s PBR module has a `StandardMaterial`
which supports many properties of materials
but might use a slightly different name or sematic for expressing them.
(Another point of reference is [Filament],
which is a physically based rendering engine for Android.
It also has a nice overview of [Material Properties].)

We will try to map the properties of the MaterialX files we get to `StandardMaterial`.

[MaterialX]: http://www.materialx.org/
[PBR Spec]: https://github.com/AcademySoftwareFoundation/MaterialX/blob/b26f19e75226163acea0e24b457e3d4649e04b64/documents/Specification/MaterialX.PBRSpec.md
[Bevy]: https://bevyengine.org/
[Filament]: https://google.github.io/filament/Filament.html
[Material Properties]: https://google.github.io/filament/Material%20Properties.pdf
