use crate::{Race, race};
use spacetimedb::{ReducerContext, Table};

pub fn seed(ctx: &ReducerContext) {
    if ctx.db.race().iter().next().is_none() {
        ctx.db.race().insert(Race {
            id: 1,
            name: "Human".into(),
            description: "The history of humans in Aelynmar stretches back into the mists of time, predating the cataclysm of the Shattering. They arose as diverse tribes and scattered settlements across the varied landscapes, from fertile river valleys to windswept plains. Their early societies were characterized by adaptability, a keen understanding of their local environments, and a burgeoning capacity for innovation and social organization. While not inherently magical in the way some other ancient races were, they possessed a natural curiosity about the world around them and a drive to explore and shape it to their needs. Their strength lay in their numbers, their resilience, and their ability to learn and adapt to new circumstances.".into()
        });

        ctx.db.race().insert(Race {
            id: 2,
            name: "Tormog".into(),
            description: "Long before the Veil shimmered across Aelynmar, the Tormog walked its wilder paths. With their skin the hue of twilight skies, strong limbs moving with a surprising grace, and the ivory of their lower tusks a distinctive mark, they were a people shaped by the untamed lands. Deep forests echoed with their calls, rugged mountains held their ancient settlements, and the whispering marshes knew their ways. Their societies were woven from tradition, a deep respect for the spirits that dwelled in the land, and a strength that spoke of enduring harsh seasons. They were a part of Aelynmar's tapestry, their story unfolding in rhythm with the wild heart of the world.".into()
        });

        ctx.db.race().insert(Race {
            id: 3,
            name: "Vrask".into(),
            description: " In the deep folds of Aelynmar's high peaks, where the wind whispers secrets through the stone and the roots of the world run deep, dwelled the Vrask. Even before the Veil shimmered upon the land, they were a people of the mountains, their lives measured by the slow turning of the ages and the enduring strength of the rock around them. Stout and sure-footed, with hands calloused by generations of shaping stone and working metal, they possessed a quiet mastery of the earth's hidden treasures. Their halls, carved deep within the mountains' embrace, echoed with the steady rhythm of their craft, and their lore was etched in the very veins of ore they unearthed. They were a people of steadfast tradition, their lives bound to the ancient heart of the mountains.".into()
        });

        ctx.db.race().insert(Race {
            id: 4,
            name: "Lumycus".into(),
            description: "As the Shadow of the Shattering fell upon Aelynmar and the vibrant lands of the elves faced ruin, a profound change took root within their ancient groves. Wise among the elven druids and those who read the stars recognized the grave peril to the heart-trees, the deep-rooted life of the world they cherished. In a move born of deep reverence and a desire for enduring guardianship, they undertook a solemn communion with these venerable trees. Through ritual and the weaving of their own life essence, they sought a bond that would withstand the encroaching darkness and preserve the spirit of the groves. The energies unleashed by the Shattering, the nascent Veil that now shimmered across the land, intertwined with this merging, giving rise to the Lumycus – beings of part-elf and part-tree, their lives now echoing with the subtle magic of the Veil within the forests. In this way, even as the elven realms diminished, a new guardianship for the ancient groves was established.".into()
        });
    }
}
