local rm = resourcemanager
local cube_mesh = rm.get_mesh("cube")
local cube_material = rm.get_material("test-128px")

local Cube = {}

function Cube:init(x, y, z)
    self.name = string.format("%d %d %d", x, y, z)
    self.components.Transform = components.Transform.new(x, y, z)
end

function Cube:tick()
    local xyzw = self.components.Transform:xyzw();
    local x = xyzw[1];
    local y = xyzw[2];
    local dist = math.sqrt(x*x + y*y)
    local z = math.sin(time*2+dist)*math.max(10-dist, 0)/10
    local new = components.Transform.new(x, y, z);
    --print(self.name, x, y, z, new:xyzw()[1], new:xyzw()[2], new:xyzw()[3])
    self.components.Transform = new
end

sent.register({
    name = "Cube",
    cls = Cube,
    components = {
        components.Transform.new(0, 0, 0),
        components.Renderable.new_mesh(cube_mesh, cube_material),
    },
})

local z = 0
for x=-8,8 do
    for y=-8,8 do
        Cube.new(x, y, z)
    end
end
