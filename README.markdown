Bevy + Blender Third-Person 3D Game Template
=============================================

This is a template project for developing 3D games using the [Bevy](https://bevyengine.org/) engine, with [Blender](https://www.blender.org/) as a visual editor.

It is based on [kaosat-dev's toolkit](https://github.com/sdfgeoff/blender_bevy_toolkit) and also makes use of the [Bevy asset loader plugin](https://github.com/NiklasEi/bevy_asset_loader).

I make absolutely no claims as to whether this repository's contents reflect good practices — I am just sharing the result of my trying to setup a working project. If you do see something problematic or perfectible, though, please let me know.

Building and Running
---------------------

### Prerequisites

* [Have Rust installed](https://www.rust-lang.org/tools/install).

### Actually Building and Running

`cd` into this repository, and run `cargo run`.

### Controls

Use the WASD keys to move the blue cube, Space/Shift to go up/down, and the arrow keys to rotate the camera.

Note that there are no physics, so you will pass through objects in the scene.

Using in Your Own Project
--------------------------

### Prerequisites

* [Have Rust installed](https://www.rust-lang.org/tools/install) (obiously).
* [Have Blender installed](https://www.blender.org/download/) (obviously).
* Install and activate the [gltf_auto_export](https://github.com/kaosat-dev/Blender_bevy_components_workflow/blob/main/tools/gltf_auto_export/README.md) Blender plugin.

### Using Blender

* Open the `assets/blueprints/scene.glb` file with Blender.
* To add a new object blueprint named `myobject`, select the `library` scene and create a new collection `myobject` in it. In this collection, add an empty "Plain axes" named `myobject_components`, and create your object's mesh, etc.
* To add the object to the scene, switch to the `scene` scene, and "Add" a new "Collection Instance" of your object.
* Then save the Blender project. If you have the autosave plugin installed and activated, this should save a bunch of .glb files (one for your scene, and one for each object) in the `assets/blueprints` directory (one for the scene, plus one for each object in the scene).
* Open the `src/assets.rs` file, find the `pub struct GameAssets` declaration. For each .glb file (say, `myobject.glb`) in the blueprints directory, add the corresponding two lines to the declaration. For our `myobject` example:
```rust
#[asset(path = "blueprints/myobject.glb")]
pub myobject: Handle<bevy::gltf::Gltf>,
```

