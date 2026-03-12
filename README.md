# steamworks.js

Forked. Used by [SeizeControl](https://seizecontrol.app/). Requires Steam installed and running.

## Installation

```bash
# From GitHub
npm install github:AlifPapp/steamworks.js

# For Electron/NW.js projects (match --target to your Electron version)
npm install github:AlifPapp/steamworks.js --runtime=electron --target=27.0.0
npm install github:AlifPapp/steamworks.js --runtime=node-webkit --target=0.75.0
```

Or use as a local path dependency (e.g. `"steamworks.js": "file:./steamworks.js"` in the parent app's package.json).

For Electron: steamworks.js is a native module and cannot be used by default in the renderer process. Configure `main.js`:

```js
const mainWindow = new BrowserWindow({
    webPreferences: {
        contextIsolation: false,
        nodeIntegration: true
    }
})
```

Call `electronEnableSteamOverlay()` at the end of `main.js` for the Steam overlay:

```js
require('steamworks.js').electronEnableSteamOverlay()
```

For production builds, copy files from `sdk/redistributable_bin/win64/` (`.node` addon and `steam_api64.dll`) into the build output. electron-forge may need extra config to bundle them.

## API

```js
const steamworks = require('steamworks.js')

// You can pass an appId, or don't pass anything and use a steam_appid.txt file
const client = steamworks.init(480)

// Print Steam username
console.log(client.localplayer.getName())

// Tries to activate an achievement
if (client.achievement.activate('ACHIEVEMENT')) {
    // ...
}
```

See `client.d.ts` for the full API documentation.

## How to build

You only need to build if you are changing steamworks.js code.

Prerequisites: [node.js](https://nodejs.org/en/), [Rust](https://www.rust-lang.org/tools/install) 1.80+, [Clang](https://rust-lang.github.io/rust-bindgen/requirements.html), and [Steam](https://store.steampowered.com/about/) installed and running.

```sh
npm install
npm run build:debug
```

### Testing Electron

```sh
cd test/electron && npm install && npm start
```

Click "activate overlay" to test.
