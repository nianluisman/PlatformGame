# Editing levels on the tile grid

Open `level_one.tmx` in Tiled. Every cell is 32 × 32 pixels, but you only
work with grid indexes such as column `12`, row `10`; Tiled handles pixels.

Paint with the three tiles from the `Logic` tileset:

- `Collisions`: paint the green Solid tile anywhere the player can stand.
- `Hazards`: paint the red Spike tile where a spike should appear.
- `Spawns`: paint one orange PlayerStart tile where the player begins.

The top-left cell is `(0, 0)`. X increases right and Y increases downward.
Keep the layer names unchanged; they are case-sensitive.

Tiled's `id`, `nextlayerid`, and `nextobjectid` values are editor metadata.
Gameplay ignores them completely. Let Tiled manage these values; IDs do not
need to be sequential and you never need to renumber existing layers.

The logic layers create gameplay entities rendered by the game with simple
colored shapes. Later, decorative art layers can be added separately.
