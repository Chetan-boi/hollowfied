# ============================================================
#  BLEACH: Hollowfied — Keymap Reference
#  (read-only reference — keybindings are hardcoded in src/main.rs)
# ============================================================

## Movement
| Key(s)                  | Action              |
|-------------------------|---------------------|
| W / Arrow Up            | Move Up             |
| S / Arrow Down          | Move Down           |
| A / Arrow Left          | Move Left           |
| D / Arrow Right         | Move Right          |

## Combat
| Key(s)                        | Action                                  | Reiatsu Cost    |
|-------------------------------|-----------------------------------------|-----------------|
| Space / J / Left Click        | Getsuga Tensho (projectile)             | 16 (0 Hollowfied) |
| Left Shift / Right Shift      | Shunpo (Flash Step) — hold move dir    | 14              |

## Forms  (transform between Ichigo's power states)
| Key   | Form                             | HP    | Rei   | Speed   | Notes                    |
|-------|----------------------------------|-------|-------|---------|--------------------------|
| 1     | Shikai (Normal / Zangetsu)       | 100   | 100   | 175     | Balanced                 |
| 2     | Bankai (Tensa Zangetsu)          | 150   | 120   | 218     | Default start form       |
| 3     | Hollowfied (Vaizard / Vasto)     | 200   | 80    | 236     | Free Getsuga, tankiest   |
| Tab   | Cycle forms (Normal→Bankai→Hollow) | —   | —     | —       | Same as pressing 1/2/3   |

## Debug / Dev
| Key | Action                    |
|-----|---------------------------|
| H   | Take 15 damage (test)     |
| R   | Heal 25 HP (test)         |

## Tips
- **Shunpo** only triggers when you're holding a movement key + press Shift.
- **Hollowfied** form costs 0 Reiatsu for Getsuga — spam freely but you have less Rei for Shunpo.
- **Invincibility frames**: 0.8 s after taking a hit — use Shunpo to escape during that window.
- **Grimmjow charges** at you — sidestep with Shunpo.
- **Ulquiorra teleports** rather than chasing — don't get cornered.

