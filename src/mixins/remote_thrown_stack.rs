// HEAD of `remote_projectile_item` (game/network/remote_projectiles.rs), which a server's
// entity data 8 reaches for a projectile that is no potion. `ThrowableItemProjectile`'s
// DATA_ITEM_STACK is the stack a snowball, egg or ender pearl was thrown as: keep it, as
// `set_item` keeps a fireball's. The original body then runs on, for fireballs only.
fn remote_thrown_stack(
    projectiles: &mut horizon_client::join_seq::JoinList<Projectile, false>,
    id: u32,
    stack: &ItemStack,
) {
    let Some(i) = projectiles.iter().position(|p| {
        p.id == id
            && matches!(
                p.kind,
                ProjectileKind::Snowball | ProjectileKind::Egg | ProjectileKind::EnderPearl
            )
    }) else {
        return;
    };
    projectiles.update(i, |p| p.set_thrown_item(stack));
}
