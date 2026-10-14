local Sounds = {
  -- The .wav files these entries referenced (beep4, button9, beep6, digitout)
  -- are not present in this archive, so each definition only produced a
  -- LoadSoundBuffer error at startup. Left empty so the engine falls back to
  -- its own defaults for IncomingChat, MultiSelect and MapPoint.
  SoundItems = {},
}

return Sounds
