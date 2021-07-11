import flatbuffers
import Q3DModel.Model
import Q3DObject.Mesh

import bpy


def export(context, objects, name='all'):
    from abrasion.blender.addon import console_print

    console_print(context, "Starting export...")
    path = bpy.data.filepath
    if path == "":
        raise Exception("File must be saved")
    path += f".{name}.q3dm"

    builder = flatbuffers.Builder(1024)
    depsgraph = context.evaluated_depsgraph_get()

    nodes = []

    for o in objects:
        console_print(context, f"{o.type}: {o.name}")
        if o.type != 'MESH':
            console_print(context, " ... skipped.")
            continue
        mesh = o.evaluated_get(depsgraph).data
        mesh.calc_loop_triangles()

        fverts = []
        for vertex in mesh.vertices:
            Q3DObject.Vertex.VertexStart(builder)
            position = Q3DObject.Vector3.CreateVector3(
                builder,
                x = vertex.co.x,
                y = vertex.co.y,
                z = vertex.co.z,
            )
            Q3DObject.Vertex.VertexAddPosition(builder, position)
            normal = Q3DObject.Vector3.CreateVector3(
                builder,
                x = vertex.normal.x,
                y = vertex.normal.y,
                z = vertex.normal.z,
            )
            Q3DObject.Vertex.VertexAddNormal(builder, normal)
            fverts.append(Q3DObject.Vertex.VertexEnd(builder))
        nverts = len(fverts)

        Q3DObject.Mesh.MeshStartVerticesVector(
            builder, len(fverts))
        for ivert in reversed(range(len(fverts))):
            builder.PrependUOffsetTRelative(fverts[ivert])
        fverts = builder.EndVector()

        ntris = len(mesh.loop_triangles)
        Q3DObject.Mesh.MeshStartTrianglesVector(
            builder, ntris)
        for triangle in mesh.loop_triangles:
            Q3DObject.ITriangle.CreateITriangle(
                builder,
                triangle.vertices[0],
                triangle.vertices[1],
                triangle.vertices[2],
            )
        ftris = builder.EndVector()
        console_print(context, f" ... {nverts} vertices, {ntris} triangles")

        Q3DObject.Mesh.MeshStart(builder)
        Q3DObject.Mesh.MeshAddItriangles(builder, ftris)
        Q3DObject.Mesh.MeshAddVertices(builder, fverts)
        fmesh = Q3DObject.Mesh.MeshEnd(builder)


        Q3DModel.Node.NodeStart(builder)
        Q3DModel.Node.NodeAddMesh(builder, fmesh)
        c0 = o.matrix_world.col[0]
        c1 = o.matrix_world.col[1]
        c2 = o.matrix_world.col[2]
        c3 = o.matrix_world.col[3]
        Q3DModel.Node.NodeAddTransform(builder, Q3DModel.Matrix4.CreateMatrix4(
            builder,
            c0.x, c0.y, c0.z, c0.w,
            c1.x, c1.y, c1.z, c1.w,
            c2.x, c2.y, c2.z, c2.w,
            c3.x, c3.y, c3.z, c3.w,
        ))
        nodes.append(Q3DModel.Node.NodeEnd(builder))


    Q3DModel.Node.NodeStartChildrenVector(builder, len(nodes))
    for node in nodes:
        builder.PrependUOffsetTRelative(node)
    nodes = builder.EndVector()

    Q3DModel.Node.NodeStart(builder)
    Q3DModel.Node.NodeAddTransform(builder, Q3DModel.Matrix4.CreateMatrix4(
        builder,
        1, 0, 0, 0,
        0, 1, 0, 0,
        0, 0, 1, 0,
        0, 0, 0, 1,
    ))
    Q3DModel.Node.NodeAddChildren(builder, nodes)
    root = Q3DModel.Node.NodeEnd(builder)

    Q3DModel.Model.ModelStart(builder)
    Q3DModel.Model.ModelAddRoot(builder, root)
    model = Q3DModel.Model.ModelEnd(builder)

    builder.Finish(model, file_identifier=b'Q3DM')
    buf = builder.Output()
    with open(path, 'wb') as f:
        f.write(buf)
    console_print(context, f"Saved to {path}")


