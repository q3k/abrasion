Blender/Abrasion Addon
===

Building / Installing
---

To build:

    bazel buld //blender:addon

Then symlink/copy `bazel-bin/blender/addon` into Blender's `scripts/addons/abrasion-addon`. **DO NOT INSTALL THE ADDON AS `abrasion`, THIS BREAKS IMPORT PATHS**/.

Usage
---

Right click on a collection in the ourliner and select 'Export Abrasion Q3DM' to recursively export this collection as a q3dm file. It will be saved next to your .blend file, with a '.(collectionName).q3dm' suffix.
