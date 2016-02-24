pub mod position;
pub mod sprite;
pub mod tint;
pub mod velocity;

components! {
    struct GameComponents {
        #[hot] position: position::Position,
        #[hot] velocity: velocity::Velocity,
        #[hot] tint: tint::Tint,
        #[hot] sprite: sprite::Sprite,
    }
}
