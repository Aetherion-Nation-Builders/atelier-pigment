# Atelier Pigment app icon

**The hidden sun behind a sweep of pigment**: an ivory disc rising, crossed by one thick stroke
of ochre, on a volcanic black field. Three colours, from the Nation's palette:

| Colour | Hex | Used for |
|---|---|---|
| Volcanic black | `#141210` | the field, and the gap that separates the stroke from the sun |
| Ivory | `#efe6d2` | the sun |
| Ochre | `#d9a441` | the pigment |

## Geometry

A 512-unit tile (`viewBox="0 0 512 512"`), rounded square with `rx=112`, no border, as all the
Atelier icons. The macOS renders pad it onto Apple's 824/1024 icon grid; the Windows and Linux
renders crop 22 units off each side so the mark reads at 16–48 px. `photocraft.svg` is the
master; the file names keep their upstream `photocraft` names so packaging stays untouched.

## Provenance

Drawn for Atelier Pigment by hand as SVG (2026). It replaces PhotoCraft's kitsune icon, which
stays in PhotoCraft's own repository. License: see `LICENSE.txt`.

## Regenerating

`packaging/icons.sh` (needs resvg; iconutil on macOS for the .icns). The committed renders were
made from the same SVG with cairosvg and Pillow.
