# Encore UI redesign

The desktop app now shares the website's visual identity. The replay library uses a light surface, a persistent collection/game sidebar, and media-first cards. The editor uses a dark version of that same palette for video, audio mixing and trimming.

## Design direction

- Palette: Onyx `#0c120c`, Brick Ember `#c20114`, Blue Slate `#5d737e`, Pale Sky `#bfd5e2`, Ghost White `#ecebf3`.
- Type: locally bundled Manrope for controls and reading; Barlow Condensed for library/dialog headings. Font licenses are included alongside the assets.
- Layout: left-aligned navigation and content, compact recording controls, and a stable coordinate system shared by the timeline ruler and tracks.
- Principles: replays lead; red signals primary actions and recording; flat, quiet controls replace the old metal textures. No remote font requests or decorative motion.

```text
encore.     recorder status / capture mode       shortcut / settings
------------------------------------------------------------------
Your library | Your replays.                       Create montage
All replays  | Search                      Sort / grid / list
Favorites    | ---------------------------------------------------
Games        | Today / game
             | [replay]           [replay]           [replay]
             | title + actions    title + actions    title + actions
```

Design review: the website's oversized marketing headings are reduced for desktop use. The library keeps its recognizable type and palette; editing keeps a dark stage and compact controls. The single display heading provides the personality, while the controls remain practical. No recording, encoding, storage schema, IPC contract, installer version or release publishing logic changes.

## Verification

```powershell
npm --prefix apps/ui ci
npm --prefix apps/ui run build
cd apps/ui
npx playwright install chromium
npm run test:ui
```

Tests cover library navigation/search, list/grid actions, montage ordering, editor mixing/trim/export payloads, settings and focus handling, recorder states, and layout at desktop minimum size and narrow windows. Screenshots are saved under the ignored `apps/ui/test-results/` directory.



For local visual review with the website's existing replay assets, optionally set `ENCORE_TEST_MEDIA_DIR` to its absolute `assets` directory before running tests. Fixtures are served by the test harness and are never bundled into the app.



