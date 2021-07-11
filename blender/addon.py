import importlib

import bpy

from abrasion.blender.export import export


def console_print(context, *args, **kwargs):
    for a in context.screen.areas:
        if a.type != 'CONSOLE':
            continue

        c = {}
        c['area'] = a
        c['space_data'] = a.spaces.active
        c['region'] = a.regions[-1]
        c['window'] = context.window
        c['screen'] = context.screen
        s = " ".join([str(arg) for arg in args])
        for line in s.split("\n"):
            line = '[abrasion] ' + line
            bpy.ops.console.scrollback_append(c, text=line)


class OUTLINER_OT_collection_abrasion(bpy.types.Operator):
    """Foo."""
    bl_idname = "outliner.collection_abrasion"
    bl_label = "Export Abrasion Q3M"

    @classmethod
    def poll(cls, context):
        return context.collection is not None

    def execute(self, context):
        export(
            context,
            context.collection.all_objects.values(),
            context.collection.name,
        )
        return {'FINISHED'}

def menu_func(self, context):
    layout = self.layout
    layout.separator()
    layout.operator(OUTLINER_OT_collection_abrasion.bl_idname)

def register():
    bpy.utils.register_class(OUTLINER_OT_collection_abrasion)
    bpy.types.OUTLINER_MT_collection.append(menu_func)
    print("Abrasion: Registered.")

def unregister():
    bpy.utils.unregister_class(OUTLINER_OT_collection_abrasion)
    bpy.types.OUTLINER_MT_collection.remove(menu_func)
    print("Abrasion: Unregistered.")
