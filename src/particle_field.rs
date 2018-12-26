use super::particle::Particle;
use super::mass::Mass;

struct ParticleField {
    field: Vec<Mass>,
}

impl ParticleField {
    pub fn time_step(&self) -> ParticleField {
        ParticleField {
            field: self.field.iter().map(|x| x.time_step()).collect()
        }
    }
}
