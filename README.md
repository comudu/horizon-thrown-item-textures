# Thrown Item Textures

A native mod for [Horizon](https://github.com/horizon-rust/horizon) (Minecraft: Java Edition 26.3 in Rust):
**thrown snowballs, eggs and ender pearls look like the item they were thrown as**, as in vanilla.

In vanilla a snowball given its own look (`/give @s snowball[item_model="my:snowball"]`,
`custom_model_data` with a resource pack, a CIT pack, ...) keeps that look in flight. In Horizon it
always flies as the plain `item/snowball` sprite. This mod draws the projectile's own stack with its
item model, the way Horizon already draws a thrown potion.

> 한국어 설명은 [아래](#한국어)에 있어요.

## Install

You need the Horizon launcher with native mods allowed. A native mod runs with your rights, outside any
sandbox: read the source first. It is all in [`horizon-mod.toml`](horizon-mod.toml) and [`src/mixins/`](src/mixins/).

**From GitHub (recommended).** In the launcher's Mods page choose **Add from GitHub** and paste

```text
https://github.com/comudu/horizon-thrown-item-textures
```

or run `horizon-launcher mods add-github https://github.com/comudu/horizon-thrown-item-textures --trust-native-code`.
Then choose **Build** (or run `horizon-launcher native-build --instance <instance> --trust-native-code`) and play.
`horizon-launcher mods update-github thrown_item_textures` picks up new releases.

**By hand.** Download `thrown_item_textures.hmod` from [Releases](https://github.com/comudu/horizon-thrown-item-textures/releases),
put it in the game folder's `horizon-native-mods/` (the folder that holds `options.txt`; create it if missing), add
the line `mod.thrown_item_textures:true` to `horizon-mods.txt` in the same folder, then **Build** in the launcher.

## Compatibility

| | |
| --- | --- |
| Horizon build | Nightly 2026-11-20 (`0a875da91dbb0fdcb7ba8c42c11341aa229af51a`) |
| horizon-mixins | `e59da9094f0c2fdda6c627fe9eff22dec6a906f0` |
| Side | Client only (single player included). Works with the Mannequin Armor mod |

## How it works

Three mixins (the build log says `Function replacement: 3 functions patched by 3 mixins` with this mod alone):

1. **`keep-thrown-stack`** (`MODIFY_ARG` on `Game::spawn_projectile`): Horizon kept the thrown stack only for
   eggs. A snowball or ender pearl thrown by the player, a dispenser or a mod now keeps it too
   (`ThrowableItemProjectile.setItem`).
2. **`remote-thrown-stack`** (`HEAD` on `remote_projectile_item`): on a server, the projectile's entity data 8
   (`DATA_ITEM_STACK`) was read for fireballs only; it is now kept for snowballs, eggs and ender pearls.
3. **`draw-thrown-stack`** (cancellable `HEAD` on `Game::thrown_item_geometry`): a snowball, egg or pearl that has
   a stack is drawn as that stack's item model (ground display, facing the camera, as `ThrownItemRenderer`).
   Without one it is drawn as before.

So blue and brown eggs also fly as blue and brown eggs now.

## Not covered

- A burning thrown snowball/egg/pearl that has a stack doesn't draw the fire over it (Horizon's flame helper is
  private to another module).
- Projectiles already in flight when the mod is turned on, and Horizon worlds' snowballs saved without an item,
  keep the old sprite.

## License

MIT, see [LICENSE](LICENSE). Not affiliated with Mojang, Microsoft or the Horizon project.

## 한국어

마크에서는 눈덩이에 커스텀 텍스처(`item_model`, `custom_model_data` + 리소스팩, CIT 등)를 입히면 던졌을 때도 그
모양으로 날아가는데, Horizon에서는 항상 기본 눈덩이 그림으로만 보이는 문제를 고치는 네이티브 모드예요.
눈덩이, 달걀(파란/갈색 달걀 포함), 엔더 진주가 던진 아이템 그대로의 모델로 그려져요. 싱글, 서버 둘 다 돼요.

**설치:** 런처 Mods 페이지에서 **Add from GitHub**를 누르고 `https://github.com/comudu/horizon-thrown-item-textures`를 붙여넣은 다음 **Build** 하면 돼요.
직접 넣으려면 [Releases](https://github.com/comudu/horizon-thrown-item-textures/releases)에서 `thrown_item_textures.hmod`를 받아서 게임 폴더의
`horizon-native-mods/`에 넣고, `horizon-mods.txt`에 `mod.thrown_item_textures:true`를 추가한 뒤 Build 하세요.

**호환:** Horizon Nightly 2026-11-20 (`0a875da`) 전용, 클라이언트 전용이에요. 마네킹 갑옷 모드랑 같이 써도 돼요.

**안 되는 것:** 커스텀 눈덩이가 불에 붙었을 때 불꽃 효과는 안 보여요. 모드를 켜기 전에 이미 날아가던 눈덩이는 예전 그림 그대로예요.
