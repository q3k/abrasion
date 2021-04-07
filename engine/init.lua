print("Hello, Lua!")
for k,v in pairs(components) do
    print("Lua Component", k, v)
end

local sent = {}
sent.register = function (cfg)
    if cfg.name == nil then
        error("sent.register: needs name")
    end
    if cfg.cls == nil then
        error("sent.register: needs cls")
    end
    local name = cfg.name
    local cls = cfg.cls
    local components = cfg.components or {}

    if cls.__sent_class_id ~= nil then
        error(string.format("sent.register: %s already registered", name))
        return
    end

    -- Recreate config when calling native function, to ensure no metatable
    -- fuckery.
    local sent_class_id = __sent_register({
        name = name,
        cls = cls,
        components = components,
    })

    cls.__sent_class_id = sent_class_id
    cls.new = function(...) 
        local arg = {...}
        local res = __sent_new(sent_class_id)
        if res.init ~= nil then
            res:init(unpack(arg))
        end
        return res
    end
end

local Test = {}

function Test:init(val)
    self.val = val
end

function Test:tick()
    print("tick! " .. tostring(self.val))
    print("components " .. tostring(self.components))
end

sent.register({
    name = "Test",
    cls = Test,
    components = {
        components.Transform.new(0, 0, 0),
    },
})

local t1 = Test.new(123)
t1:tick()
local t2 = Test.new(234)
t2:tick()
