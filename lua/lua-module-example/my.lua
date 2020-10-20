local _M = {}

local i

local function add(n)
    i = i + n
end

function _M.new(name, num)
    print("new "..name)
    i = num
end

function _M.add(n)
    add(n)
end

function _M.get()
    return i
end

return _M
