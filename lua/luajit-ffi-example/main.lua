local ffi = require "ffi"

local myffi = ffi.load("myffi")
ffi.cdef[[
void *malloc(size_t size);
void free(void *ptr);
int printf(const char *fmt, ...);
void init_memory(size_t size);
void set_value(int v);
void print_values();
void free_memory();
int *malloc_convert(void *p);
]]


-- 完全由c语言管理内存
myffi.init_memory(10)
for i = 1, 10 do
    myffi.set_value(i)
end

myffi.print_values()
myffi.free_memory()

-- luajit自动管理内存 大小有限制 内存自动释放
local int_array_t = ffi.typeof("int[?]")
local array_v = ffi.new(int_array_t, 100)
local i = 0
while i < 100 do
    array_v[i] = i + 1
    i = i + 1
end
print(array_v[99])


-- ffi.C管理内存, p复制nil会自动回收内存
local p = ffi.gc(ffi.C.malloc(100 * ffi.sizeof("int")), ffi.C.free)
-- 需要手动释放
-- local p = ffi.C.malloc(100*4)
-- 方法1 利用外部库做转换
-- local p_pointer = myffi.malloc_convert(p)
local int_pointer_t = ffi.typeof("int *")
-- cast转换注意参数不要写反, 否则报错: size of C type is unknown or too large
local p_pointer = ffi.cast(int_pointer_t, p)

for j = 0, 99 do
    p_pointer[j] = j + 1
end

local x = p_pointer[98]
-- print(type(x))
print(p_pointer[98])
-- C.printf 需要类型转换
local c_int = ffi.typeof("int")
ffi.C.printf("HHH: %d\n", ffi.cast(c_int, p_pointer[98]))
-- 手动释放
-- ffi.C.free(ffi.gc(p, nil))
-- 自动释放
p = nil
