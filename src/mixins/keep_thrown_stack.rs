// The argument of `self.add_projectile(projectile)` in `Game::spawn_projectile`
// (game/items/projectile_spawned.rs). A snowball or ender pearl keeps the stack it was thrown as
// (`ThrowableItemProjectile.setItem`: one of it), as an egg already does through `set_egg_item`.
// A projectile that already has its item, and every other kind, is left as it was.
fn keep_thrown_stack(projectile: Projectile, stack: &ItemStack) -> Projectile {
    let mut projectile = projectile;
    if projectile.item.is_none()
        && matches!(
            projectile.kind,
            horizon_client::projectiles::ProjectileKind::Snowball
                | horizon_client::projectiles::ProjectileKind::Egg
                | horizon_client::projectiles::ProjectileKind::EnderPearl
        )
    {
        projectile.set_thrown_item(stack);
    }
    projectile
}
