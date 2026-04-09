use serde::Deserialize;
use crate::objects::classic_enemy::ClassicEnemy;

#[derive(Debug, Deserialize)]
pub(crate) struct Wave{
    enemies : Vec<ClassicEnemy>,
    max_enemies : i32,
    max_duration : f32,
}