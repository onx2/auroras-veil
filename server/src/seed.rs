//! A module used to define the static data in the database and provides a function
//! to seed that data on database start up.

use crate::{Class, Race, class, race};
use spacetimedb::{ReducerContext, Table};

pub fn seed_static_data(ctx: &ReducerContext) {
    seed_race(ctx);
    seed_class(ctx);
}

fn seed_race(ctx: &ReducerContext) {
    if ctx.db.race().iter().next().is_none() {
        ctx.db.race().insert(Race {
            id: 1,
            name: "Human".into(),
            description: "The history of humans in Aelynmar stretches back into the mists of time, predating the cataclysm of the Shattering. They arose as diverse tribes and scattered settlements across the varied landscapes, from fertile river valleys to windswept plains. Their early societies were characterized by adaptability, a keen understanding of their local environments, and a burgeoning capacity for innovation and social organization. While not inherently magical in the way some other ancient races were, they possessed a natural curiosity about the world around them and a drive to explore and shape it to their needs. Their strength lay in their numbers, their resilience, and their ability to learn and adapt to new circumstances.".into()
        });

        ctx.db.race().insert(Race {
            id: 2,
            name: "Tormŏg".into(),
            description: "Long before the Veil shimmered across Aelynmar, the Tormŏg walked its wilder paths. With their skin the hue of twilight skies, strong limbs moving with a surprising grace, and the ivory of their lower tusks a distinctive mark, they were a people shaped by the untamed lands. Deep forests echoed with their calls, rugged mountains held their ancient settlements, and the whispering marshes knew their ways. Their societies were woven from tradition, a deep respect for the spirits that dwelled in the land, and a strength that spoke of enduring harsh seasons. They were a part of Aelynmar's tapestry, their story unfolding in rhythm with the wild heart of the world.".into()
        });

        ctx.db.race().insert(Race {
            id: 3,
            name: "Vrask".into(),
            description: " In the deep folds of Aelynmar's high peaks, where the wind whispers secrets through the stone and the roots of the world run deep, dwelled the Vrask. Even before the Veil shimmered upon the land, they were a people of the mountains, their lives measured by the slow turning of the ages and the enduring strength of the rock around them. Stout and sure-footed, with hands calloused by generations of shaping stone and working metal, they possessed a quiet mastery of the earth's hidden treasures. Their halls, carved deep within the mountains' embrace, echoed with the steady rhythm of their craft, and their lore was etched in the very veins of ore they unearthed. They were a people of steadfast tradition, their lives bound to the ancient heart of the mountains.".into()
        });

        ctx.db.race().insert(Race {
            id: 4,
            name: "Lümycus".into(),
            description: "As the Shadow of the Shattering fell upon Aelynmar and the vibrant lands of the elves faced ruin, a profound change took root within their ancient groves. Wise among the elven druids and those who read the stars recognized the grave peril to the heart-trees, the deep-rooted life of the world they cherished. In a move born of deep reverence and a desire for enduring guardianship, they undertook a solemn communion with these venerable trees. Through ritual and the weaving of their own life essence, they sought a bond that would withstand the encroaching darkness and preserve the spirit of the groves. The energies unleashed by the Shattering, the nascent Veil that now shimmered across the land, intertwined with this merging, giving rise to the Lümycus – beings of part-elf and part-tree, their lives now echoing with the subtle magic of the Veil within the forests. In this way, even as the elven realms diminished, a new guardianship for the ancient groves was established.".into()
        });
    }
}

fn seed_class(ctx: &ReducerContext) {
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
