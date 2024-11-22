# BADAQ

Repositori pengembangan sistem DAQ external balance dengan load cell.

## Sebelum Pengembangan

Pasang [Rust](https://www.rust-lang.org/tools/install) sesuai dengan sistem operasi. Apabila sudah terpasang, perlu memperhatikan syarat minimum untuk pengembangan di tautan [INI](https://tauri.app/start/prerequisites/).

pengembangan aplikasi menggunakan [TAURI](https://tauri.app/). Untuk itu, perlu *command line interface* ([CLI](https://tauri.app/reference/cli/)) yang disediakan, seperti:

```bash
cargo install tauri-cli --version "^2.0.0" --locked
```

Dari sisi *frontend*, diperlukan **Deno**. Pemasangan dapat dilakukan sesuai tautan [instalasi](https://docs.deno.com/runtime/#install-deno). Setelah terpasang, jalankan instruksi berikut:

```bash
deno install
```

Buat file `.env.development.local` dalam direktori yang sama dengan file `vite.config.js` berada. Adapun isi dari file sama dengan (modifikasi seperlunya bagian setelah simbol '=') isi file `.env.development`.

Buat juga file `.env` dalam direktori `src-tauri` dengan isi sama dengan (modifikasi seperlunya bagian setelah simbol '=') file `.env.development`.

## Mulai Pengembangan

Dengan asumsi persyaratan sudah terpenuhi, jalankan instruksi berikut di terminal:

```bash
deno task tauri dev
```
