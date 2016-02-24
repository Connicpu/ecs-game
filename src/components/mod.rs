pub use self::position::Position;
pub use self::sprite::Sprite;
pub use self::tint::Tint;
pub use self::velocity::Velocity;

pub mod position;
pub mod sprite;
pub mod tint;
pub mod velocity;

components! {
    struct GameComponents {
        #[hot] position: position::Position,
        #[hot] velocity: velocity::Velocity,
        #[hot] sprite: sprite::Sprite,
        #[cold] tint: tint::Tint,
        #[cold] camera_follow: (),
    }
}
