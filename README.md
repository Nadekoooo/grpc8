# Refleksi modul 8

---

## 1. Apa perbedaan utama antara unary, server streaming, dan bi-directional streaming RPC, dan dalam skenario apa masing-masing paling sesuai?

Unary RPC hanya mengirim satu permintaan dan menerima satu respons. Cocok digunakan untuk operasi sederhana seperti `getById` atau `insert data`. Server streaming mengembalikan banyak respons untuk satu permintaan, cocok untuk log histori, data periodik, atau laporan progres. Sementara bi-directional streaming memungkinkan pertukaran pesan dua arah secara simultan, sangat ideal untuk aplikasi seperti live chat, call center virtual, atau sistem monitoring real-time.

---

## 2. Apa pertimbangan keamanan yang perlu diperhatikan saat mengimplementasikan layanan gRPC di Rust, khususnya terkait otentikasi, otorisasi, dan enkripsi data?

Keamanan dapat dijaga melalui TLS untuk mengenkripsi komunikasi, implementasi otentikasi (contohnya JWT atau OAuth2), dan otorisasi berbasis peran. Selain itu, penting juga untuk menghindari eksploitasi input pengguna, menghindari race condition, dan menggunakan dependency yang aman dan terverifikasi. Rust memberikan keuntungan tambahan karena compile-time safety yang kuat.

---

## 3. Apa tantangan atau masalah yang mungkin muncul saat menangani bidirectional streaming di Rust gRPC, terutama pada aplikasi seperti chat?

Tantangan utama termasuk manajemen state koneksi yang bersifat asinkron, potensi kebocoran channel, serta kesulitan sinkronisasi antara pesan masuk dan keluar. Dalam aplikasi chat, penting juga menjaga urutan pesan dan mengelola client yang disconnect tanpa pemberitahuan eksplisit.

---

## 4. Apa kelebihan dan kekurangan menggunakan `tokio_stream::wrappers::ReceiverStream` untuk streaming respons di layanan Rust gRPC?

**Kelebihan**:
- Mudah digunakan dengan `mpsc::channel`
- Terintegrasi penuh dengan `tonic` dan async ecosystem
- Cocok untuk server streaming sederhana

**Kekurangan**:
- Kurang fleksibel untuk use-case kompleks
- Debugging sulit jika stream gagal diam-diam
- Tidak cocok untuk alur komunikasi yang sangat dinamis atau stateful

---

## 5. Bagaimana kode gRPC Rust dapat disusun agar mendukung penggunaan ulang dan modularitas, serta memudahkan pemeliharaan dan pengembangan?

Gunakan struktur modular:
- Pisahkan setiap service (payment, transaction, chat) ke dalam modul terpisah
- Abstraksikan logika ke dalam trait dan implementasi
- Tempatkan `.proto` file dalam folder `proto/`
- Gunakan layer `services/`, `handlers/`, dan `models/` seperti arsitektur MVC
Ini akan memudahkan testing, refactor, dan pengembangan skala besar.

---

## 6. Dalam implementasi `MyPaymentService`, langkah tambahan apa yang mungkin dibutuhkan untuk menangani logika pemrosesan pembayaran yang lebih kompleks?

- Validasi input secara menyeluruh
- Logging dan audit trail untuk transaksi
- Integrasi dengan third-party API seperti payment gateway
- Manajemen retry jika terjadi error jaringan
- Penanganan status transaksi (pending, failed, success)
- Penyimpanan transaksi ke database

---

## 7. Apa dampak adopsi gRPC terhadap arsitektur sistem terdistribusi, terutama dalam hal interoperabilitas dengan teknologi dan platform lain?

gRPC membawa standar komunikasi yang efisien dan ketat berkat Protocol Buffers, namun interoperabilitas memerlukan kompiler `.proto` untuk berbagai bahasa. Hal ini bagus untuk ekosistem yang seragam tapi memerlukan upaya lebih untuk platform yang hanya mengenal REST/JSON. Namun, keuntungannya termasuk performa tinggi, kontrak eksplisit, dan auto-codegen lintas bahasa.

---

## 8. Apa kelebihan dan kekurangan HTTP/2 sebagai protokol dasar gRPC dibandingkan HTTP/1.1 atau HTTP/1.1 + WebSocket?

**Kelebihan HTTP/2**:
- Multiplexing (banyak stream dalam satu koneksi TCP)
- Header compression
- Bi-directional streaming bawaan

**Kekurangan**:
- Kompleksitas debugging lebih tinggi
- Tidak semua proxy/network support dengan baik
- Kurang familiar bagi developer yang terbiasa dengan REST

---

## 9. Bagaimana model request-response dari REST API berbeda dengan kemampuan bidirectional streaming dari gRPC dalam hal komunikasi real-time?

REST bersifat synchronous: client mengirim permintaan dan menunggu balasan. Ini tidak efisien untuk real-time. gRPC bi-directional streaming memungkinkan pertukaran data dua arah yang terus-menerus tanpa koneksi ulang, sangat cocok untuk notifikasi, chat, atau dashboard real-time.

---

## 10. Apa implikasi pendekatan berbasis skema pada gRPC (menggunakan Protocol Buffers) dibanding pendekatan tanpa skema seperti JSON pada REST API?

Pendekatan skema memastikan validasi tipe data yang ketat, autogenerasi kode, dan efisiensi ukuran payload. Namun, JSON lebih fleksibel, mudah dibaca manusia, dan tidak perlu definisi eksplisit. gRPC cocok untuk sistem besar dan stabil, sedangkan REST lebih cocok untuk integrasi cepat dan fleksibel.

---
