use crate::{Class, class};
use spacetimedb::{ReducerContext, Table};

pub fn seed(ctx: &ReducerContext) {
    if ctx.db.class().iter().next().is_none() {
        ctx.db.class().insert(Class {
            id: 1,
            name: "Myrmidon".into(),
            description: "A disciplined and versatile warrior who excels at both offense and defense. They are masters of tactical combat, using their skills to disrupt enemy formations, control the battlefield, and deliver precise, powerful strikes. They can specialize in different weapon styles and combat stances.".into()
        });

        ctx.db.class().insert(Class {
            id: 2,
            name: "Templar".into(),
            description: "The Templar is a holy warrior, a paragon of divine power and martial skill. Channeling the light of the divine, they strike down enemies with righteous fury while shielding their allies from harm. Whether engaging in brutal combat, protecting their comrades, or lifting their spirits with divine blessings, the Templar stands as an unwavering force in the face of darkness.".into()
        });

        ctx.db.class().insert(Class {
            id: 3,
            name: "Shaman".into(),
            description: "The Shaman is a spiritual leader who communes with the natural world, drawing upon the power of spirits and the land to protect allies, weaken enemies, and alter the flow of battle. Through their deep connection with the spiritual realm, Shamans heal the wounded, buff their allies, and debuff their foes. They wield runes and totems, casting nature-infused magic. Their unique blend of healing, support, and control makes them invaluable members of any adventuring group.".into()
        });

        ctx.db.class().insert(Class {
            id: 4,
            name: "Occultist".into(),
            description: "The Occultist is a dark spellcaster who delves into the forbidden and often dangerous aspects of magic. With a mastery over shadowy forces, curses, and summoning unholy creatures, they wield corrupted power to weaken, manipulate, and destroy their enemies from afar. Whether summoning demonic entities to fight on their behalf or casting debilitating curses to drain the life from foes, the Occultist thrives in sowing chaos and fear, weakening their enemies before delivering a final, devastating blow of dark magic.".into()
        });

        ctx.db.class().insert(Class {
            id: 5,
            name: "Stalker".into(),
            description: "The Stalker is a versatile and agile fighter who strikes swiftly and silently. Blending ranged attacks with rapid melee strikes, Stalkers excel at taking down their foes before they can react. Masters of stealth, they use the environment to their advantage, disappearing into the shadows and ambushing unsuspecting targets. Whether using bows for precise shots or daggers and throwing knives for close-range combat, Stalkers are unpredictable and deadly.".into()
        });

        ctx.db.class().insert(Class {
            id: 6,
            name: "Arcanist".into(),
            description: "The Arcanist is a master of arcane magic, harnessing the immense power drawn from the very fabric of the world itself. With unparalleled control over elemental forces, they unleash devastating spells, shape the environment to their will, and manipulate the arcane energies that flow through the world of Aelynmar. Whether raining down torrents of fire, freezing enemies with ice, or summoning storms of destruction, the Arcanist is a force to be reckoned with, controlling the battlefield with their vast array of elemental powers.".into()
        });
    }
}
