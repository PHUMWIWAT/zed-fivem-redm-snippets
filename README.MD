# FiveM RedM Snippets

A [Zed](https://zed.dev) extension for FiveM and RedM Lua. It adds snippets while you type, and it gives Lua Language Server the CitizenFX definitions in `plugin/`.

This project is not made or endorsed by Cfx.re. The definitions in `plugin/` come from [overextended/fivem-lls-addon](https://github.com/overextended/fivem-lls-addon) (MIT).

## Requirements

- [Zed](https://zed.dev)
- Zed's **Lua** extension, so `.lua` files are the Lua language and `lua-language-server` is available

## Install

The extension is not in Zed's extension list yet. Clone this repository and install it as a dev extension:

```powershell
git clone https://github.com/PHUMWIWAT/zed-fivem-redm-snippets.git
```

Open the command palette and run `zed: install dev extension`. Choose the cloned folder, then reopen Zed.

## Snippets

Open a `.lua` file and type one of these prefixes. The snippet appears in the completion list.

| Prefix | Inserts |
| --- | --- |
| `CreateThread` | `CreateThread` loop with `Wait` |
| `Citizen.CreateThread` | The same loop as `Citizen.*` |
| `RegisterNetEvent` | A net event registration |
| `AddEventHandler` | An event handler |
| `RegisterCommand` | A command |
| `TriggerServerEvent` | A server event |
| `TriggerClientEvent` | A client event |
| `TriggerEvent` | A local event |
| `RegisterNUICallback` | An NUI callback |
| `PerformHttpRequest` | An HTTP request |
| `fxmanifest` | An `fxmanifest.lua` header for GTA V |
| `fxmanifest-rdr3` | An `fxmanifest.lua` header for RedM |
| `client_script` | A client script line |
| `server_script` | A server script line |
| `shared_script` | A shared script line |
| `vector3` | `vector3(x, y, z)` |
| `PlayerPedId` | The local ped and its coords |

Edit `snippets/lua.json` to change them, then install the dev extension again.

## Natives

In a resource `.lua` file, type a function name such as `RegisterCommand`. Lua Language Server completes it from the definitions in `plugin/`. Those files only tell the editor the names. The game still calls the functions when the resource runs.

By default the extension loads `plugin/library/runtime` and `plugin/library/natives/CFX-NATIVE`. It does not run `plugin/plugin.lua`.

To also load game natives, set `game` to `gtav` or `rdr3` in your Zed settings:

```json
{
  "lsp": {
    "fivem-redm": {
      "settings": {
        "game": "gtav"
      }
    }
  }
}
```

`gtav` adds `plugin/library/natives/GTAV`. `rdr3` adds `plugin/library/natives/RDR3`. Any other value keeps the default libraries.

After you install the extension or change `game`, install the dev extension again, reopen Zed, and run `editor: restart language server`.
