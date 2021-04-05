print("Hello, Lua2!")
--for k,v in pairs(components) do
--    print("Component", k, v)
--end

local sent = {}
sent.register = function (name, cls)
    if cls.__sent_class_id ~= nil then
        print("Attempting to re-register " .. name)
        return
    end
    local sent_class_id = __sent_register(name, cls)
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
end

sent.register("Test", Test)
local t1 = Test.new(123)
t1:tick()
local t2 = Test.new(234)
t2:tick()
