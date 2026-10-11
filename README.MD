# FiveM RedM Snippets

ส่วนขยาย Zed ที่ใส่ snippet ของสคริปต์ Lua สำหรับ FiveM และ RedM ส่วนขยายนี้ไม่ได้จัดทำหรือรับรองโดย Cfx.re

## สิ่งที่ต้องมี

- [Zed](https://zed.dev)
- ส่วนขยาย **Lua** ของ Zed เพื่อให้ไฟล์ `.lua` ถูกจับเป็นภาษา Lua

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
