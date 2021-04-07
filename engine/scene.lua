local rm = resourcemanager
local cube_mesh = rm.get_mesh("cube")
local cube_material = rm.get_material("test-128px")

local Test = {}

function Test:init()
end

function Test:tick()
end

sent.register({
    name = "Test",
    cls = Test,
    components = {
        components.Transform.new(0, 0, 0),
        components.Renderable.new_mesh(cube_mesh, cube_material),
    },
})

Test.new()
