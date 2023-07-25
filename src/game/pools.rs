use rand::prelude::*;

pub trait Pickable<T> {
    fn pick(&self, rng: &mut impl rand::Rng) -> T;
}

impl<T: Copy> Pickable<T> for [(T, i32)] {
    /// picks an item from the pool based on the provided weights.
    fn pick(&self, rng: &mut impl rand::Rng) -> T {
        self.choose_weighted(rng, |item| item.1)
            .unwrap()
            .0
            .to_owned()
    }
}

pub mod tiles {
    use crate::resources::isometric;
    pub const GRASS: [(isometric::Index, i32); 3] = [
        (isometric::Index::GrassShort, 2),
        (isometric::Index::GrassMedium, 1),
        (isometric::Index::GrassMedium, 2),
    ];

    pub const SOIL: [(isometric::Index, i32); 5] = [
        (isometric::Index::Soil, 5),
        (isometric::Index::SoilBlades, 2),
        (isometric::Index::SoilLeaves, 1),
        (isometric::Index::SoilCracked, 5),
        (isometric::Index::SoilCrackedAlt, 5),
    ];

    pub const WATER: [(isometric::Index, i32); 3] = [
        (isometric::Index::Water, 5),
        (isometric::Index::WaterWavy, 1),
        (isometric::Index::WaterWavyLight, 2),
    ];

    pub const DIRT: [(isometric::Index, i32); 2] = [
        (isometric::Index::Dirt, 10),
        (isometric::Index::DirtMedium, 1),
    ];
}
