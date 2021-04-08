sent = {}
sent.newindex = function(t, k, v)
    return __sent_components_newindex(t, k, v)
end
sent.index = function(t, k)
    return __sent_components_index(t, k)
end
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

require("//engine/scene.lua")
