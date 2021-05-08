components = {}
local components_metatable = {}
setmetatable(components, components_metatable)
components_metatable.__index = function (_, idstr)
    return function (...)
        return __component_construct({
            idstr = idstr,
            args = {...},
        })
    end
end

sent = {}
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

    if cls.__sent_class_name ~= nil then
        error(string.format("sent.register: %s already registered", name))
        return
    end

    -- Recreate config when calling native function, to ensure no metatable
    -- fuckery.
    __sent_class_register({
        name = name,
        cls = cls,
        components = components,
    })

    cls.__sent_class_name = name
    cls.spawn = function(...) 
        local arg = {...}

        -- Make object table, instantiate with runtime.
        local table = {}
        local sent_id = __sent_spawn({
            class_name = name,
            instance = table,
        })
        table.__sent_id = sent_id

        -- Make 'tick' trampoline, used by runtime to do fast call into ticker
        -- without bind/argument push.
        table.__sent_tick = function()
            if table.tick ~= nil then
                table:tick()
            end
        end

        -- Configure components dispatcher.
        table.components = {}
        table.components.__sent_id = sent_id
        local components_meta = {}
        components_meta.__index = function(t, k)
            return __sent_components_index(sent_id, k)
        end
        components_meta.__newindex = function(t, k, v)
            return __sent_components_newindex(t, k, v)
        end
        setmetatable(table.components, components_meta)

        -- Make table deref via table to class.
        local metatable = {}
        metatable.__index = cls
        setmetatable(table, metatable)

        -- Call initializer, if present.
        if table.init ~= nil then
            table:init(unpack(arg))
        end

        return table
    end
end

require("//engine/scene.lua")
