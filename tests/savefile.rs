use std::{fs, io};

use bloodborne_savedata::{
    gaitem::{Equipment, GaItem},
    gem::{GemKind, GemShape},
    item::{Item, ItemKind},
    savefile::Savefile,
    slot::SlotShape,
};

const USERDATA: &str = "./tests/data/userdata0000";
const SAVES_DIR: &str = "./tests/data/bb-save-editor";

fn load() -> Savefile {
    Savefile::read(USERDATA).unwrap()
}

/// Every save that is not intentionally broken or an export
fn samples() -> Vec<std::path::PathBuf> {
    let mut paths: Vec<_> = fs::read_dir(SAVES_DIR)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| fs::metadata(path).unwrap().len() >= 0x20000 && !path.ends_with("noappearancesave0"))
        .collect();

    paths.push(USERDATA.into());
    paths.push("./tests/data/userdata0001".into());
    paths
}

#[test]
fn read() -> io::Result<()> {
    let save = Savefile::read(USERDATA)?;
    assert_eq!(save.stats.level, 27);
    assert_eq!(save.stats.vitality, 7);
    assert_eq!(save.stats.endurance, 8);
    assert_eq!(save.stats.strength, 14);
    assert_eq!(save.stats.skill, 13);
    assert_eq!(save.stats.bloodtinge, 33);
    assert_eq!(save.stats.arcane, 9);

    assert_eq!(save.stats.blood_echoes, 1765);
    assert_eq!(save.stats.insight, 6);

    Ok(())
}

#[test]
fn read_other_values() {
    let save = load();

    assert_eq!(save.name, "Bloodletter Buil");
    assert_eq!(save.death_count, 2);
    assert_eq!(save.new_game_cycle, 0);
    assert_eq!(Savefile::read("./tests/data/userdata0001").unwrap().new_game_cycle, 1);
    assert_eq!(save.playtime, 5353590);
    assert_eq!(save.position.map, (21, 0, 0, 0));
}

#[test]
fn broken_saves_are_errors() {
    for name in ["emptysave", "no_inv_save", "no_inv_end_save", "noappearancesave0"] {
        assert!(Savefile::read(format!("{SAVES_DIR}/{name}")).is_err(), "{name}");
    }
}

#[test]
fn writing_without_changes_gives_the_same_bytes() {
    for path in samples() {
        let save = Savefile::read(&path).unwrap_or_else(|error| panic!("{path:?}: {error}"));

        assert_eq!(save.gaitems.len(), 4096);
        assert!(save.to_bytes().unwrap() == save.bytes, "{path:?}");
    }
}

#[test]
fn items_use_ids() {
    let save = load();
    let items: Vec<&Item> = save.inventory.occupied().collect();

    let vials = items.iter().find(|item| item.kind() == ItemKind::Good && item.id() == 1000).unwrap();
    assert_eq!(vials.amount, 20);

    // Weapons and armor point to the handle section
    let bloodletter = items.iter().find(|item| item.kind() == ItemKind::Weapon && item.id() == 29000300).unwrap();
    let Some(GaItem::Equipment(equipment)) = save.gaitems.iter().find(|entry| entry.handle() == Some(bloodletter.first_part)) else {
        panic!("no entry for the weapon");
    };
    assert!(equipment.is_weapon());
    assert_eq!(equipment.id, 29000300);
    assert_eq!(equipment.slots[0].shape, SlotShape::Radial);
    assert_ne!(equipment.slots[0].gem, 0);
    assert_eq!(equipment.slots[2].shape, SlotShape::Closed);

    assert!(items.iter().any(|item| item.kind() == ItemKind::Armor && item.id() == 230000));
    assert!(save.keys.occupied().any(|item| item.id() == 4000 || item.kind() == ItemKind::Good));
    assert!(save.inventory.occupied().any(|item| item.kind() == ItemKind::Gem));
}

#[test]
fn gems_have_typed_shapes() {
    let save = load();
    let shapes: Vec<_> = save
        .gaitems
        .iter()
        .filter_map(|entry| match entry {
            GaItem::Gem(gem) => Some(gem.kind),
            _ => None,
        })
        .collect();

    assert_eq!(shapes.len(), 4);
    assert!(shapes.contains(&GemKind::Gem(GemShape::Circle)));
}

#[test]
fn simple_fields_are_written() {
    let mut save = load();

    save.name = "Hunter".into();
    save.playtime = 1234;
    save.death_count = 77;
    save.new_game_cycle = 3;
    save.stats.level = 99;
    save.stats.health = [1000; 3];
    save.position.x += 10.0;
    save.position.map = (24, 1, 0, 0);
    save.appearance[0] ^= 0xFF;

    let again = Savefile::from_bytes(save.to_bytes().unwrap()).unwrap();

    assert_eq!(again.name, "Hunter");
    assert_eq!((again.playtime, again.death_count, again.new_game_cycle), (1234, 77, 3));
    assert_eq!((again.stats.level, again.stats.health), (99, [1000; 3]));
    assert_eq!((again.position.x, again.position.map), (save.position.x, (24, 1, 0, 0)));
    assert_eq!(again.appearance, save.appearance);

    save.name = String::new();
    assert!(save.to_bytes().is_err());
    save.name = "12345678901234567".into();
    assert!(save.to_bytes().is_err());
}

#[test]
fn inventory_records_are_written() {
    let mut save = load();

    let record = save.inventory.items.iter().position(|item| item.kind() == ItemKind::Good && item.id() == 1000).unwrap();
    save.inventory.items[record].amount = 99;

    let free = save.storage.items.iter().position(|item| item.kind() == ItemKind::Empty).unwrap();
    save.storage.items[free] = Item::good(1100, 5);

    let again = Savefile::from_bytes(save.to_bytes().unwrap()).unwrap();

    assert_eq!(again.inventory.items[record].amount, 99);
    assert_eq!(again.storage.items[free].id(), 1100);
    assert_eq!(again.storage.items[free].kind(), ItemKind::Good);
    assert_eq!(again.storage.items.len(), save.storage.items.len());
}

#[test]
fn equipment_can_be_added() {
    let mut save = load();
    let size = save.bytes.len();

    let handle = save.gaitems.iter().filter_map(GaItem::handle).map(|handle| handle & 0xFFFF).max().unwrap() + 1;
    let handle = 0x8080_0000 | handle;

    let free = save.gaitems.iter().position(|entry| *entry == GaItem::EMPTY).unwrap();
    let Some(GaItem::Equipment(template)) = save.gaitems.iter().find(|entry| matches!(entry, GaItem::Equipment(_))) else {
        panic!("no equipment");
    };
    save.gaitems[free] = GaItem::Equipment(Equipment { handle, id: 8100000, ..*template });

    let record = save.inventory.items.iter().position(|item| item.kind() == ItemKind::Empty).unwrap();
    save.inventory.items[record] = Item::weapon(handle, 8100000);

    let bytes = save.to_bytes().unwrap();
    assert_eq!(bytes.len(), size);

    let again = Savefile::from_bytes(bytes).unwrap();
    assert_eq!(again.gaitems[free].handle(), Some(handle));
    assert_eq!(again.inventory.items[record].first_part, handle);
    // What was after the section moved but is the same
    assert_eq!(again.stats, save.stats);
    assert_eq!(again.name, save.name);
    assert_eq!(again.inventory.occupied().count(), save.inventory.occupied().count());
}

#[test]
fn gems_can_be_edited() {
    let mut save = load();

    let index = save.gaitems.iter().position(|entry| matches!(entry, GaItem::Gem(_))).unwrap();
    let GaItem::Gem(gem) = &mut save.gaitems[index] else { unreachable!() };
    gem.kind = GemKind::Gem(GemShape::Droplet);
    gem.effects[1] = 13102;

    let again = Savefile::from_bytes(save.to_bytes().unwrap()).unwrap();
    let GaItem::Gem(gem) = &again.gaitems[index] else { panic!() };
    assert_eq!(gem.kind, GemKind::Gem(GemShape::Droplet));
    assert_eq!(gem.effects[1], 13102);
}
