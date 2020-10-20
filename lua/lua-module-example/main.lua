local my = require "my"

my.new("Module init", 1)

local start_time = os.clock()
print(start_time)
for i = 1, 1000000 do
    my.add(i)
end
local end_time = os.clock()
print("Module number: "..my.get())
print(string.format("time: %.4f", end_time - start_time))
