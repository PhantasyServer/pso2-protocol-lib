# 0.3.0
### Changes
 - Added equip/unequip request and response packets.
 - Added some second password related packets.
 - Added `NoItem` variant to `ItemType`.
 - Added `equiped_items` to `CharacterList`.
 - Renamed `Unk040F` to `EnemyKilled`.
 - Renamed `Unk0422` to `EnemyAction`.
 - Renamed `Unk0B09` to `MinimapRevealRequest`.
 - Renamed `Unk0B13` to `MinimapReveal`.
 - Renamed `ItemPickedUp` to `DespawnObject`.
 - Renamed `RemoveObject` to `DespawnPlayer`.
 - Renamed `GetNearbyCharacters` to `GetAllianceStatus`.
 - Renamed `ObjectType::Unk10` to `ObjectType::World`.
 - Expanded fields of `DamageReceivePacket`.
 - Fixed minor doc mistakes.

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
