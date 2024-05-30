# 0.2.0
### Changes
 - packetlib_impl: Moved Windows filetime conversion to the derive function.
 - Added a few unknown packets.
 - Added some title, class change, character creation and story related packets.
 - Renamed `unk3` field of `SetLobbyEventPacket` to `repeat_secs`.
 - Renamed `unk1` field of `DamageReceivePacket` to `damage_id`.
 - Renamed `unk3` field of `Character` to `unk4`.
 - Moved `unk1` field of `ClassInfo` to `Character` as `unk3`.
 - Removed `psotime_to_duration` and `duration_to_psotime` functions.
