# BADAQ

Repositori pengembangan sistem DAQ external balance dengan load cell.

[![Get it from the Snap Store](https://snapcraft.io/en/dark/install.svg)](https://snapcraft.io/badaqmon)

## Sebelum Pengembangan

Sebelum pengembangan dimulai, ada beberapa persyaratan yang harus dilakukan,
diantaranya:

### Rust Tauri (backend)

Pasang [Rust](https://www.rust-lang.org/tools/install) sesuai dengan sistem
operasi. Apabila sudah terpasang, perlu memperhatikan syarat minimum untuk
pengembangan di tautan [INI](https://tauri.app/start/prerequisites/).

pengembangan aplikasi menggunakan [TAURI](https://tauri.app/). Untuk itu, perlu
_command line interface_ ([CLI](https://tauri.app/reference/cli/)) yang
disediakan, seperti:

```bash
cargo install tauri-cli --version "^2.0.0" --locked
```

### Deno + SolidJS (frontend)

Dari sisi _frontend_ sebagai **BADAQ Monitor**, diperlukan **Deno**. Pemasangan
dapat dilakukan sesuai tautan
[instalasi](https://docs.deno.com/runtime/#install-deno). Setelah terpasang,
jalankan instruksi berikut:

```bash
deno install
```

Buat file `.env.development.local` dalam direktori yang sama dengan file
`vite.config.js` berada. Adapun isi dari file sama dengan (modifikasi seperlunya
bagian setelah simbol '=') isi file `.env.development`.

Buat juga file `.env` dalam direktori `src-tauri` dengan isi sama dengan
(modifikasi seperlunya bagian setelah simbol '=') file `.env.development`.

### TCP Broker (simulation)

Untuk kebutuhan simulasi pengiriman paket data dari instrumen ke klien,
dibutuhkan koneksi TCP. Sehubungan dengan adanya dua klien TCP (yaitu BADAQ
Streamer dengan Qt dan BADAQ monitor) dan satu TCP server, maka TCP server harus
membuat dua _thread_ (atau lebih) untuk melayani arus data sensor. Akan tetapi,
kedua _thread_ tersebut tidak dapat saling berkomunikasi. Untuk itu lah
diperlukan penghubung melalui **MQTT broker**.

Dalam pengembangan, broker yang digunakan adalah **Eclipse Mosquitto™** dan
dapat diinstall dengan mudah melalui laman website
[berikut](https://mosquitto.org/download/).

Server **TCP Broker** dikembangkan menggunakan **Python** dan
[UV](https://docs.astral.sh/uv/) sebagai _package manager_. Instalasi **UV**
dapat dilakukan melalui laman
[berikut](https://docs.astral.sh/uv/getting-started/installation/).

### BADAQ Streamer (simulation)

**BADAQ Streamer** merupakan aplikasi yang mensimulasikan kerja dari instrumen
sebagai akuisisi data sensor ke aplikasi **BADAQ Monitor**. Dibutuhkan **Qt
Creator** untuk kompilasi aplikasi dan dapat diunduh (_open source_) melalui
laman [berikut](https://www.qt.io/offline-installers) dan pastikan juga
kompilasi dengan **CMake** tersedia.

## Mulai Pengembangan

Dengan asumsi persyaratan sudah terpenuhi, jalankan instruksi berikut di
terminal:

### MQTT Broker

Untuk kebutuhan komunikasi antar BADAQ Streamer dengan BADAQ frontend/monitor,
aktifkan terlebih dahulu **MQTT broker** sebagai berikut:

```bash
mosquitto
```

### TCP Broker (simulation)

Pindah ke direktori `src-dummy`, buat _virtual environment_ Python dengan
instruksi berikut:

```bash
uv venv --python 3.12
```

Setelah itu, aktifkan _virtual environment_ yang telah dibuat dengan instruksi
berikut:

```bash
source .venv/bin/activate
```

Untuk menjalankan **TCP Broker**, jalankan instruksi berikut diterminal:

```bash
uv run broker.py
```

> **[PERHATIAN]** Setelah **TCP Broker** berfungsi, jalankan selalu **BADAQ
> Streamer** terlebih dahulu baru disusul dengan **BADAQ Monitor**. Karena
> _thread_ pertama selalu dijadikan _producer_ MQTT. Selebihnya sebagai
> _consumer_ MQTT.

### BADAQ Streamer (simulation)

Jalankan **Qt creator** dan muat **project** dengan memilih file
`src-dummy/producer/CMAkeLists.txt`. Ikuti instruksi yang ditampilkan dan
lakukan kompilasi.

### BADAQ Monitor

Jalankan instruksi berikut di terminal

```bash
deno task tauri dev
```
