# Todo List API

API untuk aplikasi Todo List yang dibangun dengan Rust, Axum, dan MySQL.

## Persyaratan

- Rust (edisi terbaru)
- MySQL (disarankan menggunakan XAMPP)
- Cargo (termasuk dengan Rust)

## Instalasi

1. Clone repositori ini:
   ```bash
   git clone [URL_REPOSITORI]
   cd todo-list-app/api
   ```

2. Buat file `.env` di root direktori proyek:
   ```env
   DATABASE_URL=mysql://root:@localhost/todo_app
   ```

3. Buat database MySQL:
   ```sql
   CREATE DATABASE todo_app;
   ```

4. Install SQLx CLI:
   ```bash
   cargo install sqlx-cli --no-default-features --features native-tls,mysql
   ```

5. Jalankan migrasi:
   ```bash
   cd api
   sqlx migrate run
   ```

## Menjalankan Aplikasi

```bash
cargo run --bin todo-api
```

Aplikasi akan berjalan di `http://localhost:3000`

## Struktur Database

### Tabel: tasks
| Kolom | Tipe | Deskripsi |
|-------|------|-----------|
| id | INT | Primary Key, Auto Increment |
| title | VARCHAR(255) | Judul task (wajib) |
| description | TEXT | Deskripsi task (opsional) |
| completed | BOOLEAN | Status penyelesaian (default: false) |
| created_at | TIMESTAMP | Waktu pembuatan (otomatis) |
| updated_at | TIMESTAMP | Waktu pembaruan terakhir (otomatis) |

## Migrasi

### Membuat Migrasi Baru

```bash
sqlx migrate add nama_migrasi
```

### Menjalankan Migrasi

```bash
sqlx migrate run
```

### Membatalkan Migrasi Terakhir

```bash
sqlx migrate revert
```

## Pengembangan

### Fitur yang Tersedia

- [x] Koneksi ke database MySQL
- [x] Migrasi database
- [ ] CRUD operations untuk tasks
- [ ] Validasi input
- [ ] Error handling
- [ ] Testing

### Struktur Direktori

```
todo-list-app/
├── api/                    # Backend API
│   ├── src/                
│   │   ├── main.rs         # Entry point aplikasi
│   │   ├── db.rs           # Koneksi database
│   │   ├── error.rs        # Error handling
│   │   ├── repository/     # Interaksi database
│   │   └── lib.rs          # Library root
│   ├── migrations/         # File migrasi database
│   └── Cargo.toml          # Dependensi dan konfigurasi
└── README.md               # Dokumentasi ini
```

## Kontribusi

1. Fork repositori
2. Buat branch fitur (`git checkout -b fitur/namafitur`)
3. Commit perubahan (`git commit -m 'Menambahkan fitur baru'`)
4. Push ke branch (`git push origin fitur/namafitur`)
5. Buat Pull Request

## Troubleshooting

### Error "migration was previously applied but is missing"
Jika mendapatkan error ini, ikuti langkah:
1. Hapus database yang ada:
   ```bash
   mysql -u root -e "DROP DATABASE IF EXISTS todo_app;"
   ```
2. Buat database baru:
   ```bash
   mysql -u root -e "CREATE DATABASE todo_app;"
   ```
3. Hapus file migrasi yang tidak terpakai di folder `migrations/`
4. Jalankan migrasi:
   ```bash
   sqlx migrate run
   ```

## Lisensi

[MIT](LICENSE)