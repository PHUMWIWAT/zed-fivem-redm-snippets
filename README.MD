# FiveM RedM Snippets

ส่วนขยาย Zed สำหรับสคริปต์ Lua ของ FiveM และ RedM ให้ snippet ตอนพิมพ์ และส่ง definition ของ native ให้ Lua Language Server ส่วนขยายนี้ไม่ได้จัดทำหรือรับรองโดย Cfx.re definition ใน `plugin/` มาจาก [overextended/fivem-lls-addon](https://github.com/overextended/fivem-lls-addon) (MIT)

## สิ่งที่ต้องมี

- [Zed](https://zed.dev)
- ส่วนขยาย **Lua** ของ Zed เพื่อให้ไฟล์ `.lua` ถูกจับเป็นภาษา Lua และมี `lua-language-server`

## ติดตั้ง

โคลน repo แล้วติดตั้งเป็น dev extension:

```powershell
git clone https://github.com/PHUMWIWAT/zed-fivem-redm-snippets.git
```

ใน Zed เปิด command palette แล้วรัน `zed: install dev extension` เลือกโฟลเดอร์ที่โคลนมา จากนั้นเปิด Zed ใหม่

## การใช้งาน

เปิดไฟล์ `.lua` แล้วพิมพ์ prefix ด้านล่าง snippet จะโผล่ในรายการ completion

| พิมพ์ | ได้ |
| --- | --- |
| `CreateThread` | ลูป `CreateThread` + `Wait` |
| `Citizen.CreateThread` | ลูปแบบ `Citizen.*` |
| `RegisterNetEvent` | ลงทะเบียน net event |
| `AddEventHandler` | handler ของ event |
| `RegisterCommand` | คำสั่ง |
| `TriggerServerEvent` | ยิงไปเซิร์ฟเวอร์ |
| `TriggerClientEvent` | ยิงไปไคลเอนต์ |
| `TriggerEvent` | ยิงในเครื่องเดียวกัน |
| `RegisterNUICallback` | callback ของ NUI |
| `PerformHttpRequest` | เรียก HTTP |
| `fxmanifest` | หัว `fxmanifest.lua` สำหรับ GTA V |
| `fxmanifest-rdr3` | หัว `fxmanifest.lua` สำหรับ RedM |
| `client_script` | บรรทัด client script |
| `server_script` | บรรทัด server script |
| `shared_script` | บรรทัด shared script |
| `vector3` | `vector3(x, y, z)` |
| `PlayerPedId` | ped ของผู้เล่นกับ coords |

แก้หรือเพิ่ม snippet ได้ที่ `snippets/lua.json` แล้วติดตั้ง dev extension ซ้ำ

## Native

เปิดไฟล์ `.lua` ใน resource แล้วพิมพ์ชื่อฟังก์ชันตรง ๆ เช่น `RegisterCommand` Lua Language Server จะเติมจาก definition ใน `plugin/` ค่าเริ่มต้นคือ `runtime` กับ `CFX-NATIVE`

definition นี้มีไว้ให้ตัวแก้ไขรู้จักชื่อฟังก์ชัน เกมเป็นคนเรียกตอนรัน resource

ถ้าต้องการ native ของเกมด้วย ให้ตั้ง `game` เป็น `gtav` หรือ `rdr3`:

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

จากนั้นรัน `editor: restart language server` ถ้าเพิ่งติดตั้งหรือเพิ่งเปลี่ยนเกม ให้ติดตั้ง dev extension ซ้ำแล้วเปิด Zed ใหม่ก่อน restart language server
