# Bloodborne savedata findings

Everything here was found by reading the structure of the original save editor (`Bloodborne-save-editor-main`) and checking it against the sample saves in `tests/data`. Facts verified by the tests are stated plainly; guesses are marked as such.

Terms: **name** is the offset found for the character name (see [Locating things](#locating-things)), most offsets below are relative to it. All values are little endian. The savefile has a fixed size (1,310,720 bytes for `userdata0000`) and the editor never touches a checksum.

## Header

- Always seems to begin with `41` at `0x00`
- Always seems to be `54` at `0x0C`
- Always seems to be `54` at `0x14`
- `0x04`..`0x07`: last loaded map. `m21_00_00_00` becomes `00 00 00 15`, lacking the `m`. `Position::map` is these bytes reversed, `(21, 0, 0, 0)`
- `0x08`..`0x0B`: playtime, `u32`, in milliseconds

## Locating things

Most data starts at variable offsets, but relative offsets between them are fixed.

1. Search for the ASCII bytes `FACE`, starting from `0xF000` (in every known save it is after that). The face data (`0xEB` bytes) follows it
2. inventory start = face data start - 4 - 34028
3. name = inventory start - 469
4. storage start = inventory start + `0x85DC`
5. key items start = name + `0x7DC9` (32201)
6. `LCED` (`4C 43 45 44`), searched after `FACE`, is the anchor for the position
7. AOB = name + 68545, see [Flags](#flags-relative-to-aob)

A save without `FACE` (`noappearancesave0`) is unusable, and so is a file shorter than `0xF000`.

## Handle section ("GaItem")

Starts at `0x54` (84) and holds **4096 entries**: every gem, weapon and armor piece the character ever got. Inventories only point at these by handle. The length of an entry is decided by its 4th byte:

| 4th byte | Entry | Length |
|---|---|---|
| `0x00` | empty, normally `00 00 00 00 FF FF FF FF` | 8 |
| `0xC0` | gem / rune | 40 |
| `0x80` | weapon | 60 |
| `0x90` | armor | 60 |

- In every sample the section ends 151 bytes before the name (4 bytes before the health, which is at name - 147)
- In every sample the gems come first and are contiguous, then weapons / armor and empties are mixed
- Finding the end of the gems by matching known kind / shape pairs every 40 bytes stops early on `testsave4` and `testsave5` (2 and 20 gems found, there are 115 and 105). Walking by type byte is reliable
- `testsave4` has gems with kind `0x1F204` and shape `0x1F284`. The game doesn't use these, the library keeps them as `Unknown`

### Gem / rune (40 bytes)

| Offset | Meaning |
|---|---|
| 0 | handle, `u32`, `0xC080xxxx`. Inventories and slots refer to it |
| 4 | source, `u32` |
| 8 | kind, `u32`: `1` gem, `2` rune |
| 12 | shape, `u32` |
| 16 | 6 effect ids, `u32` each. `0xFFFFFFFF` is no effect |

Shapes of gems: `0x01` radial, `0x02` triangle, `0x04` waning, `0x08` circle, `0x3F` droplet. Shapes of runes: `0x01` plain, `0x02` oath.

- The first effect decides the name, level and rating a gem is shown with. Gem effect ids are 5 digits, eg. `13101` is Bloodtinge Blood Gemstone (1), "Add blood ATK +0.5". The reference editor knew 12,489 gem effects and 83 rune effects. Rune ids are 7 digits, eg. `1100000` is "Moon"
- Transforming a gem into a rune (or back) in the reference rewrote kind, shape and effects with a default (`1100000` / `13101`), leaving handle and source alone
- Droplet gems can be put in a slot of any shape (seen in `userdata0001`, `testsave3`, `testsave8`)

### Weapon / armor (60 bytes)

| Offset | Meaning |
|---|---|
| 0 | handle, `u32`: 2 bytes of counter, `0x80`, then `0x80` for a weapon or `0x90` for armor |
| 4 | id. Armor has `0x10` as its highest byte, weapons carry level / imprint, see [Items](#items) |
| 8 | durability. `0xC8` for something freshly added |
| 12 | unknown, `0` when added |
| 16 | `1`, seems to always be that. The reference calls it the start of the slots |
| 20 | 5 slots, 8 bytes each: shape `u32`, handle of the gem in it `u32` (`0` for none) |

Slot shapes use the gem shape values, `0x80000000` is a closed slot. Equipped **runes** are stored in slots as well (note from the reference).

Adding a weapon / armor in the reference: take the highest handle in use + 1, replace the first 8 byte empty entry with a 60 byte block, so everything after it moves by 52 bytes, then cut 52 bytes off the end of the file. The inventory record is written first, as it moves too.

## Inventories

Inventory, storage and key items are arrays of **16 byte records**:

| Offset | Meaning |
|---|---|
| 0 | running number, `u32` |
| 4 | first part |
| 8 | second part |
| 12 | amount |

An empty record has first part `0`, second part `0xFFFFFFFF` and amount `0`. Empty records come after the used ones. (The earlier notes said "the first 4 bytes are the item count" and "ID taking up the last 4 bytes", which is this layout shifted by 4.)

What a record holds is decided by the highest byte of its parts:

| Kind | First part | Second part |
|---|---|---|
| Good | id, highest byte `0xB0` | id, highest byte `0x40` |
| Weapon | handle, highest byte `0x80` | id with level and imprint |
| Armor | handle, highest byte `0x90` | id, highest byte `0x10` |
| Gem / rune | handle, highest byte `0xC0` | source |

Goods and armor ids use 3 bytes. A new character automatically gets `4` separate `Bare Fists` weapons, as well as weapon id `1000`.

Sizes:
- Inventory: 1983 usable records. A 1984th would overlap the first 12 bytes of the key items, which start at inventory + 31732 (1983 * 16 + 4)
- Storage: 1984 records. Source: https://www.bloodborne-wiki.com/2024/02/full-storage-glitch.html
- Key items: 2204 bytes, which is 137 records and 12 trailing bytes. Key items can't be stored, so there is no key storage

### Counters

Inventory and storage each have two counters (`u32`):

| | First | Second |
|---|---|---|
| Inventory | name + 453 (inventory start - 16) | name + 34257 |
| Storage | name + 34737 | name + 68541 |

The reference added 1 to both whenever it added a record. What they count is unknown. The first storage counter is at the same place as the `number` of the first storage record.

### Items

- Weapon id = base id + imprint * 10000 + level * 100. Level is 0 to 10, imprint `1` is Uncanny and `2` is Lost. Example: `5121000` is a Lost +10 `5100000`
- Two weapons have `8` in the imprint digit as a part of their base id: `12080000` (Bloody Logarius' Wheel) and `6180000` (Bloody Ludwig's Rifle)
- Damage at level L is `base + base / 10 * L`, at level 10 it is `base * 2`
- Changing level / imprint of a weapon means changing the second part of the inventory record **and** the id of the handle section entry. Changing the id of armor or a weapon is the same two places. For goods only the record, both parts
- Key items and chalices always have an amount of 1
- The reference's data: 125 consumables, 27 materials, 50 key items, 25 chalices (depth 1 to 5, area Pthumeru / Loran / Isz / Hintertomb), 97 weapons (80 right hand, 17 left hand, with quicksilver bullets per shot and damage of 5 types), 255 armor pieces (head / chest / hands / legs or none, defense against physical, blunt, thrust, blood, arcane, fire, bolt, resistance to slow poison, rapid poison, frenzy, beasthood)

## Character data (relative to name)

The name text starts at **name + 1**.

| Offset | Meaning |
|---|---|
| +1 | name, UTF-16, 16 code units at most (32 bytes). `Bloodborne` is `42 00 6C 00 6F 00 6F 00 64 00 62 00 6F 00 72 00 6E 00 65 00` |
| -147 | health, `u32`, stored 3 times in a row |
| -119 | stamina, `u32`, stored 3 times in a row |
| -103 | vitality |
| -95 | endurance |
| -79 | strength |
| -71 | skill |
| -63 | bloodtinge |
| -55 | arcane |
| -35 | insight |
| -23 | level |
| -19 | blood echoes |
| +35 | gender |
| +36 | voice |
| +39 | origin |

- The 3 copies of health / stamina can differ (`testsave1`), so they have to be kept apart. Presumably current / maximum / something else, not verified
- The reference only handled the lowest byte of the attributes, the library reads and writes 4 bytes. In the samples these are the same
- Gender, voice and origin were only ever handled as raw numbers by the reference. The library doesn't expose them

## Position

After `LCED`, search for `FF FF FF FF` followed by 8 bytes of `00`. The 3 `f32` coordinates x, y, z follow right after these 12 bytes.

## Flags (relative to AOB)

AOB is name + 68545. The name comes from the reference editor; it's an anchor, not a verified byte pattern.

| Offset | Meaning |
|---|---|
| +286 | new game cycle, `u8` (name + 68831). 0 is the first |
| +360 | death count, `u32` |
| boss table | boss flags, see below |

A boss flag is a byte at AOB + offset. The boss is dead when the mask bits are set. The reference wrote the whole byte, which would erase other bits stored in it, so a write should use the mask.

| Boss | Offset / dead mask |
|---|---|
| Cleric Beast | 21714 / 0x08 |
| Father Gascoigne | 21727 / 0x80 |
| Vicar Amelia | 21602 / 0x80 |
| Celestial Emissary | 21839 / 0x08 |
| Ebrietas | 21852 / 0x80 |
| Blood Starved Beast | 21477 / 0x80 |
| Shadow Of Yharnam | 22227 / 0x80 |
| Rom | 22602 / 0x80 |
| Witch Of Hemwick | 21352 / 0x80 |
| One Reborn | 22352 / 0x80 |
| Martyr Logarius | 21977 / 0x80 |
| Dark Beast Paarl | 21464 / 0x08 |
| Amygdala | 22727 / 0x80 |
| Mergo's Wet Nurse | 22102 / 0x80 |
| Micolash | 22108 / 0x3F |
| Gehrman | 21102 / 0x80 |
| Moon Presence | 21108 / 0x20 |
| Ludwig | 22852 / 0x80, 6060 / 0x82, 6045 / 0x01 |
| Living Failures | 22983 / 0x20 |
| Lady Maria | 22977 / 0x80 |
| Kos | 23102 / 0x80 |
| Laurence | 22858 / 0x20 |

Full event flag support would need a flag id to byte + bit mapping, which isn't known.

## ISZ glitch

Two bytes at name + 72082. If the first is `0xFF`: a second byte below `0xC0` is a partial glitch (fixed by writing `0x30` to the second byte), exactly `0xC0` is the full glitch (fixed by writing `0xFF`). Taken from the reference, not verified. The library doesn't expose it.

## Not understood / not verified

- What the inventory counters count
- Whether the game accepts files edited with the library. Only the consistency of reading back what was written is tested
- The unknown `u32` at offset 12 of weapons / armor, and the gems with kind `0x1F204`
- The reference could put a record past the end of a full inventory, overwriting the data after it. The library doesn't do that
