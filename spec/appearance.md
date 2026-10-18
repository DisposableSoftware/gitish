# Appearance

## Requirements

| ID | Requirement | Status | Verified by |
|----|-------------|--------|-------------|
| `LOOK-1` | On start, the four Catppuccin base16 themes (Mocha, Macchiato, Frappe, Latte) are written to `<config dir>/gitish/themes/` if missing. Existing files are never overwritten. Any base16 YAML file in that folder appears in the theme picker. | done | `theme::tests::seed_themes_*`, `all_themes_after_seed_returns_named_themes` |
| `LOOK-2` | If no theme can be loaded, the app uses the built-in "Terminal Default" theme. A theme file that cannot be parsed is skipped. | done | `theme::tests::all_themes_returns_fallback_*`, `theme_list_new_with_empty_vec_inserts_fallback` |
| `LOOK-3` | `t` opens the theme picker with the active theme selected. `j`/`k` move, `Enter` applies the theme and saves it to `config.toml`, `Esc` closes it. A saved theme name that no longer exists is ignored. | done | `app::tests::picker_confirm_on_theme_item_applies_theme_and_closes`, `theme::tests::find_idx_by_name_*` |
| `LOOK-4` | The last row of the picker toggles transparency. `Enter` on it flips the setting, saves it and leaves the picker open. | done | `app::tests::toggle_transparent_flips_and_persists`, `picker_confirm_on_transparent_item_toggles_and_stays_in_picker` |
| `LOOK-5` | With `transparent = true` every background is the terminal's own, so compositor transparency shows. | done | manual |
| `LOOK-6` | With `use_nerd_fonts = false` every icon is a plain ASCII character. | done | `glyphs::tests::ascii_*` |
| `LOOK-7` | The screen is fully cleared before the next draw when an overlay (help, theme picker, confirm, loading) opens or closes, when the terminal is resized, when the theme or transparency changes, and when the file list or diff content changes. No stale cells remain. | done | `app::tests::needs_clear_*`, `app::tests::refresh_*`; manual |
| `LOOK-8` | The terminal is restored (raw mode off, main screen back) when the app exits, including on an error. | done | manual |

## Implementation notes

- `needs_clear` is the flag. The run loop handles overlays and resize centrally (`is_overlay_mode`). Any other operation that changes global appearance sets `app.needs_clear = true` itself.
- Themes lose their left padding if the picker's transparent row is mishandled (#55, closed). `LOOK-4` keeps the picker open on purpose: check the padding after a toggle.
