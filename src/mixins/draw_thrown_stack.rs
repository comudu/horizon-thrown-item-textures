// HEAD of `Game::thrown_item_geometry` (game/mobs/thrown_item_draw.rs). Vanilla's
// ThrownItemRenderer draws a snowball, egg or ender pearl as the item model of its stack
// (`ItemModelResolver.updateForNonLivingEntity(.., GROUND, ..)`), so a stack with its own
// item_model or custom_model_data, or a resource pack's model, shows on the projectile. Horizon
// draws these kinds as a fixed sprite instead. When the projectile has a stack, this draws it as
// Horizon draws a thrown potion and the function returns true (drawn). Without one (a projectile
// thrown before this mod was on, say) the original sprite path runs.
fn draw_thrown_stack(
    game: &Game<'_>,
    a: &Projectile,
    at: [f64; 3],
    c: glam::Vec3,
    packed: u32,
    out: &mut Vec<Vertex>,
) -> ::core::ops::ControlFlow<bool> {
    if !matches!(
        a.kind,
        ProjectileKind::Snowball | ProjectileKind::Egg | ProjectileKind::EnderPearl
    ) {
        return ::core::ops::ControlFlow::Continue(());
    }
    let Some(stack) = a
        .item
        .as_ref()
        .filter(|s| s.count > 0 && &*s.item != "minecraft:air")
    else {
        return ::core::ops::ControlFlow::Continue(());
    };
    let owner = super::item_draw::owner_at(at, 0.0, a.kind.id());
    let state = game.resolve_item(stack, DisplayContext::Ground, Some(&owner), 0);
    let pose = glam::Mat4::from_translation(c)
        * glam::Mat4::from_quat(game.camera_orientation())
        * glam::Mat4::from_scale(glam::Vec3::splat(a.sprite_scale()));
    game.submit_item_world(&state, pose, packed, out);
    ::core::ops::ControlFlow::Break(true)
}
