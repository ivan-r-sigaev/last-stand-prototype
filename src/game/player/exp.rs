use crate::ecs::component::Component;

#[derive(Debug, Clone, Copy)]
pub struct Exp(pub u32);

impl Component for Exp {}

impl Exp {
    pub fn level(&self) -> u32 {
        let mut total = 0;
        for level in 1.. {
            let req = u32::pow(level + 3, 2);
            total += req;
            if self.0 < total {
                return level - 1;
            }
        }
        unreachable!()
    }
}
