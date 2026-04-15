# 🚀 Stellar Notes Pro: Secured & Decentralized
### *Elevating Personal Privacy with Stellar Soroban Smart Contracts*

[![Network](https://img.shields.io/badge/Network-Stellar--Testnet-blue?style=for-the-badge&logo=stellar)](https://stellar.org)
[![Tech](https://img.shields.io/badge/Powered--By-Soroban--SDK-blueviolet?style=for-the-badge&logo=rust)](https://soroban.stellar.org)
[![Security](https://img.shields.io/badge/Security-Authenticated-green?style=for-the-badge)](https://soroban.stellar.org/docs/fundamentals-and-concepts/authorization)

**Stellar Notes Pro** adalah sistem manajemen catatan terdesentralisasi yang tidak hanya menyimpan data, tetapi juga menjamin keamanan akses menggunakan protokol autentikasi asli Soroban. Dibangun untuk memberikan kedaulatan data penuh bagi setiap individu.

---

## 🌟 Mengapa Proyek Ini Berbeda? (Modifikasi Unggulan)
Berbeda dengan aplikasi catatan blockchain standar, proyek ini telah dimodifikasi dengan fitur tingkat lanjut:

1.  **Identity-Based Security**: Menggunakan `Address.require_auth()`. Hanya pemilik alamat wallet yang sah yang dapat membuat atau menghapus catatan mereka.
2.  **On-Chain Event Logging**: Setiap aktivitas pembuatan dan penghapusan catatan memicu *Event* di blockchain, memungkinkan sinkronisasi frontend yang instan dan transparan.
3.  **Owner-Specific Query**: Fitur untuk menyaring catatan sehingga pengguna hanya melihat data milik mereka sendiri, menjaga privasi di tengah data publik.
4.  **Optimized Collision-Resistant ID**: ID catatan dihasilkan dari kombinasi *Ledger Sequence* dan *Timestamp* untuk menjamin keunikan data.

---

## ✨ Fitur Utama

| Fitur | Deskripsi Teknis |
| :--- | :--- |
| **Secured Creation** | Menghubungkan setiap catatan dengan identitas `Address` pengirim. |
| **Privacy Retrieval** | Fungsi `get_my_notes` hanya mengembalikan data relevan milik user. |
| **Authorized Delete** | Proteksi tingkat kontrak agar data tidak bisa dihapus oleh pihak lain. |
| **Event-Driven** | Mengirimkan sinyal `CREATED` & `DELETED` ke jaringan untuk integrasi DApp. |

---

## 🛠️ Detail Teknis & Deploy

- **Contract ID**: `CBLU4IUASQ4WUMOXBFLZRSBBLILGOH33GS4LUPKFBCCCMJCDQNMF7G2M`
- **Compiler**: Rust (WASM target)
- **SDK**: Soroban SDK v20.x

### Fungsi Kontrak (API):
1. `Notes(owner: Address, title: String, content: String) -> u64`
2. `get_my_notes(user: Address) -> Vec<Note>`
3. `delete_note(user: Address, id: u64) -> bool`

---

## 🚀 Roadmap Masa Depan

### 🔒 Short-Term: Privacy & UX
- **End-to-End Encryption**: Konten catatan dienkripsi secara lokal sebelum dikirim ke blockchain.
- **Tagging System**: Organisasi catatan menggunakan sistem metadata kategori.

### 🤝 Medium-Term: Collaboration
- **Shared Access**: Berbagi akses catatan antar alamat wallet dengan izin khusus.
- **Multi-sig Approval**: Catatan kelompok yang memerlukan persetujuan beberapa pihak sebelum dihapus.

### 🌐 Long-Term: Ecosystem
- **IPFS Integration**: Dukungan lampiran dokumen dan gambar terdesentralisasi.
- **DAO Governance**: Pengambilan keputusan fitur baru oleh komunitas pengguna.

---

## 💻 Panduan Pengembangan

### Build
```bash
soroban contract build