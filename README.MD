# FiveM RedM Snippets

ส่วนขยาย Zed สำหรับสคริปต์ Lua ของ FiveM และ RedM ส่งไลบรารี native และ runtime ให้ [Lua Language Server](https://github.com/LuaLS/lua-language-server) ตามโครงของ [cfxlua-vscode](https://github.com/overextended/cfxlua-vscode)

ไฟล์ type อยู่ใน `plugin/` และมาจาก [overextended/fivem-lls-addon](https://github.com/overextended/fivem-lls-addon) (สัญญาอนุญาต MIT) ส่วนขยายนี้ไม่ได้จัดทำหรือรับรองโดย Cfx.re

## สิ่งที่ต้องมี

- [Zed](https://zed.dev)
- ส่วนขยาย **Lua** ของ Zed ซึ่งเป็นตัวที่ติดตั้ง `lua-language-server`

## ติดตั้ง

โคลน repo แล้วติดตั้งเป็น dev extension:

```powershell
git clone https://github.com/PHUMWIWAT/zed-fivem-redm-snippets.git
```

ใน Zed เปิด command palette แล้วรัน `zed: install dev extension` เลือกโฟลเดอร์ที่โคลนมา

ถ้าเคยเปิดไฟล์ Lua อยู่แล้ว ให้รีสตาร์ท language server ด้วย `lua-language-server: restart language server` หรือเปิด Zed ใหม่

## การใช้งาน

เปิดไฟล์ `.lua` ของ resource แล้ว completion กับ hover ของ native จะมาจาก LuaLS

ค่าเริ่มต้นคือเกม **GTAV** ไลบรารีที่ถูกเปิดคือ:

- `plugin/library/runtime`
- `plugin/library/natives/CFX-NATIVE`
- `plugin/library/natives/GTAV`

runtime เป็น Lua 5.4 และเปิดสัญลักษณ์ของ LuaGLM เช่น `+=` และ `` ` ``

### เปลี่ยนเป็น RedM

ใส่ใน Zed settings (`zed: open settings file`):

```json
{
  "lsp": {
    "fivem-redm": {
      "settings": {
        "game": "rdr3"
      }
    }
  }
}
```

ค่า `game` ที่ใช้ได้คือ `gtav` และ `rdr3` หลังแก้แล้วรีสตาร์ท `lua-language-server`

### Snippet ของตัวเอง

เพิ่ม snippet ใน `snippets/lua.json` ไฟล์นี้ใช้กับภาษา Lua ตัวอย่าง:

```json
{
  "Create thread": {
    "prefix": "thread",
    "body": ["CreateThread(function()", "\t$0", "end)"],
    "description": "CreateThread wrapper"
  }
}
```

หลังแก้ไฟล์ snippet ให้ติดตั้ง dev extension ซ้ำ หรือรีโหลด Zed
