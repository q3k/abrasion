local rm = resourcemanager
local cube_mesh = rm.get_mesh("cube")
local cube_material = rm.get_material("test-128px")

local Cube = {}

function Cube:init(x, y, z)
    self.components.Transform = components.Transform.new(x, y, z)
end

function Cube:tick()
end

sent.register({
    name = "Cube",
    cls = Cube,
    components = {
        components.Transform.new(0, 0, 0),
        components.Renderable.new_mesh(cube_mesh, cube_material),
    },
})

for x=-2,2 do
    for y=-2,2 do
        for z=-2,2 do
            if z > -2 and z < 2 and x > -2 and x < 2 then
            else
                Cube.new(x, y, z)
            end
        end
    end
end
