-- One submission uses the engine's animated model transforms. Keep the legacy
-- renderer available when a driver cannot supply the GL4 model buffers.
local M = {}
local vao, vertex, index, instances, shader
local capacity, screenLocation, thicknessLocation = 0, -1, -1
local failed = false

function M.delete()
    if vao then vao:Delete(); vao = nil end
    if instances then instances:Delete(); instances = nil end
    if vertex then vertex:Delete(); vertex = nil end
    if index then index:Delete(); index = nil end
    if shader then gl.DeleteShader(shader); shader = nil end
    capacity = 0
end

local function disable(reason)
    failed = true
    M.delete()
    Spring.Echo("[OutlineBatch] using legacy renderer: " .. tostring(reason))
end

function M.available()
    if failed then return false end
    if shader then return true end
    local ok, reason = pcall(function()
        assert(gl.GetVBO and gl.GetVAO, "GL4 model buffers unavailable")
        shader = assert(gl.CreateShader({
            vertex = VFS.LoadFile("ModelMaterials/Shaders/outline_instanced.vert"),
            fragment = VFS.LoadFile("ModelMaterials/Shaders/outline_instanced.frag"),
            uniformInt = {tex0 = 0},
        }), gl.GetShaderLog())
        vertex = gl.GetVBO(GL.ARRAY_BUFFER, false)
        index = gl.GetVBO(GL.ELEMENT_ARRAY_BUFFER, false)
        assert(vertex:ModelsVBO() > 0 and index:ModelsVBO() > 0, "GL4 model buffers unavailable")
        screenLocation = gl.GetUniformLocation(shader, "screenXY")
        thicknessLocation = gl.GetUniformLocation(shader, "outlineThickness")
    end)
    if not ok then disable(reason); return false end
    Spring.Echo("[OutlineBatch] enabled GL4 animated model submission")
    return true
end

local function allocate(n)
    if vao then vao:Delete() end
    if instances then instances:Delete() end
    capacity = math.max(256, n * 2)
    instances = gl.GetVBO(GL.ARRAY_BUFFER, true)
    instances:Define(capacity, {
        {id=6, name="instData", size=4, type=GL.UNSIGNED_INT},
        {id=7, name="outlineEffect", size=4, type=GL.FLOAT},
    })
    vao = gl.GetVAO()
    vao:AttachVertexBuffer(vertex)
    vao:AttachIndexBuffer(index)
    vao:AttachInstanceBuffer(instances)
end

function M.draw(ids, data, vsx, vsy, thickness)
    if #ids == 0 then return true end
    local ok, reason = pcall(function()
        if #ids > capacity then allocate(#ids) end
        instances:InstanceDataFromUnitIDs(ids, 6)
        instances:Upload(data, 7)
        vao:ClearSubmission()
        vao:AddUnitsToSubmission(ids)
        gl.UseShader(shader)
        gl.Uniform(screenLocation, vsx, vsy)
        gl.Uniform(thicknessLocation, thickness)
        vao:Submit()
    end)
    if not ok then disable(reason) end
    return ok
end
return M
