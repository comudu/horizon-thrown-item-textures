//! Thrown Item Textures. The whole mod is its mixins (`[[native.mixin]]` in horizon-mod.toml,
//! helpers in `src/mixins/`, compiled inside the game functions they patch): this crate answers
//! no hook.
use mod_api::Hooks;

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl Hooks for Mod {}
