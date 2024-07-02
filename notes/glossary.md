# Glossary

## General terms

**CG**:
Computer Graphics.

**MaterialX**:
A specification for a file format that can be used to transport rich material descriptions between applications.

**Look**:
A collection of materials, lights, and other properties that define the appearance of a scene.
In MaterialX, a look is a collection of geometries and their materials (which in turn are made from different shaders).

**Shader**:
A program that runs on the GPU and is used to calculate the color of a pixel.

**Ubershader**:
A big shader that can be compiled with many different flags and/or variable substitutions.
[History](https://dolphin-emu.org/blog/2017/07/30/ubershaders/)

**Spatially-varying data**:
For example, color, vector, or scalar data that varies across a surface.
Pretty much a fancy way of saying "data that changes depending on where you are"
or an array of data that is indexed by a spatial coordinate.

**BSDF**:
Bidirectional Scattering Distribution Function.
A function that describes how light is scattered at a surface.
([Wikipedia](https://en.wikipedia.org/wiki/Bidirectional_scattering_distribution_function))

**USD**:
Universal Scene Description.
A file format that can be used to transport rich scene descriptions between applications.
([Wikipedia](https://en.wikipedia.org/wiki/Universal_Scene_Description))

**GLTF**:
GL Transmission Format.
A file format for 3D scenes and models.
([Wikipedia](https://en.wikipedia.org/wiki/GlTF))

**Face set**:
A set of faces in a geometry.
A face is a flat surface that is part of the boundary of a solid object.
([Wikipedia](<https://en.wikipedia.org/wiki/Face_(geometry)>))

**PBR**:
Physically Based Rendering.

**PBS**:
Physically Based Shading.

**EDF**:
Emission Distribution Function.

## MaterialX terms

**Element**:
The basic building block of MaterialX. An element can be a node, a value, a connection, or a parameter.
An element has a name, and can have child elements and attributes (named property).

**Node**:
A function that takes input values and produces output values. A node can be a shader node, a texture node, a math node, etc.
MaterialX defines a set of standard nodes that can be used to describe materials.

**Pattern**:
A simple node.

**Shader**:
A node that generates BSDF/lighting data.

**Material**:
A node which references shaders with data streams and values.

**Node Graph**:
Acyclic graph of nodes.

**Stream**:
Flow of spatially-varying data between nodes.

**Layer**:
1-, 2-, 3- or 4-channel color plane in an image file.
(Image files that don't support multiple layers are assumed to have one layer called "rgba".)

**Channel**:
A single channel of a layer. A float value of e.g. the red channel.

**Geometry**:
A renderable object.

**Partition**:
A specific named renderable subset of a geometry, e.g. a face set.

**Collection**:
A recipe for building a list of geometries.
Can be used to assign materials to geometries.

**Target**:
A software environment that interprets and renders MaterialX.

**`<materialassign>`**:
A MaterialX element that assigns a material to a geometry.

## Sources

- [MaterialX: An Open Standard for Network-Based CG Object Looks](https://github.com/AcademySoftwareFoundation/MaterialX/blob/b26f19e75226163acea0e24b457e3d4649e04b64/documents/Specification/MaterialX.Specification.md), Version 1.39 (preview), from May 9, 2024
- Github Copilot
- Others where noted
