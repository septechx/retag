---@meta

-- =========================
-- Enums
-- =========================

---@alias MimeType
---| "Png"
---| "Jpeg"
---| "Tiff"
---| "Bmp"
---| "Gif"

-- =========================
-- Structs
-- =========================

---@class Picture
---@field data string   -- binary data (u8[])
---@field mime MimeType

---@class AlbumData
---@field title string
---@field artist string
---@field cover Picture|nil

---@class TrackData
---@field title string
---@field artist string
---@field album AlbumData
---@field year integer
---@field genre string
---@field comment string
---@field composer string
---@field album_artist string
---@field disc_number integer
---@field track_number integer
---@field duration number

-- =========================
-- API
-- =========================

---@class RetagAPI
local retag = {}

---Register a callback that is called for every track.
---
---The callback receives a TrackData table.
---
---@param callback fun(data: TrackData)
function retag.register(callback) end

return retag
