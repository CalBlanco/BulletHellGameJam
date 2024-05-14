use std::f32::consts::TAU;

use bevy_hanabi::prelude::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Explosion(pub Handle<EffectAsset>);

/// Stolen setup from github examples https://github.com/rust-adventure/asteroids/blob/main/src/meteors.rs
pub fn setup(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    
    let spawner = Spawner::once(100.0.into(), false);

    let writer = ExprWriter::new();

    let age = writer.lit(0.).expr();
    let init_age =
        SetAttributeModifier::new(Attribute::AGE, age);
    let lifetime = writer.lit(1.5).expr();
    let init_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME,
        lifetime,
    );

    let drag = writer.lit(2.).expr();
    let update_drag = LinearDragModifier::new(drag);

    let color = writer.prop("spawn_color").expr();
    let init_color =
        SetAttributeModifier::new(Attribute::COLOR, color);

    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::Y).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        radius: writer.lit(64.).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocityCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        speed: (writer.lit(200.)
            * writer.rand(ScalarType::Float))
        .expr(),
    };

    let effect = effects.add(
        EffectAsset::new(32768, spawner, writer.finish())
            .with_name("explosion")
            .with_property(
                "spawn_color",
                0xFFFFFFFFu32.into(),
            )
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .init(init_color)
            .update(update_drag)
            .render(SetSizeModifier {
                size: Vec2::splat(3.).into(),
                screen_space_size: true,
            }),
    );

    commands
        .spawn((
            ParticleEffectBundle::new(effect)
                .with_spawner(spawner),
            EffectProperties::default(),
        ))
        .insert(Name::new("effect:meteor_explosion"));
}