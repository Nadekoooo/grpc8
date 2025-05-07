#  Refleksi module 8

---

## 1. Apa perbedaan utama antara unary, server streaming, dan bi-directional streaming RPC, dan dalam skenario apa masing-masing paling sesuai?

Unary RPC adalah metode paling sederhana dalam gRPC, yaitu client mengirim satu permintaan dan menerima satu respons dari server. Ini mirip dengan REST API pada umumnya, sangat cocok digunakan untuk operasi CRUD biasa. Server streaming memungkinkan server mengirimkan banyak respons untuk satu permintaan dari client, seperti pada fitur histori transaksi atau notifikasi berkelanjutan. Bi-directional streaming lebih kompleks, memungkinkan client dan server saling mengirim pesan tanpa perlu menunggu respons secara bergantian. Hal ini sangat ideal untuk aplikasi yang memerlukan komunikasi real-time seperti layanan chat atau game multiplayer. Perbedaan utama ketiganya terletak pada jumlah dan arah data yang mengalir dalam sesi RPC. Dengan memilih jenis RPC yang sesuai, efisiensi dan responsivitas aplikasi dapat ditingkatkan secara signifikan.

---

## 2. Apa pertimbangan keamanan yang perlu diperhatikan saat mengimplementasikan layanan gRPC di Rust, khususnya terkait otentikasi, otorisasi, dan enkripsi data?

Salah satu aspek paling penting dalam implementasi layanan gRPC adalah menjaga keamanan komunikasi antar layanan. Dalam konteks Rust, penggunaan TLS wajib untuk mengenkripsi data yang dikirimkan melalui jaringan. Selain itu, otentikasi pengguna bisa dilakukan menggunakan token seperti JWT (JSON Web Token) untuk memverifikasi identitas. Setelah otentikasi berhasil, otorisasi menentukan apakah pengguna tersebut berhak mengakses metode tertentu. Data yang masuk juga perlu divalidasi dengan ketat untuk mencegah serangan injeksi atau data korup. Karena Rust tidak memiliki garbage collector, keamanan memori sudah cukup terjaga, namun race condition masih bisa terjadi jika tidak hati-hati dalam penanganan asynchronous. Secara keseluruhan, aspek keamanan perlu dirancang secara menyeluruh dari awal agar sistem tetap andal.

---

## 3. Apa tantangan atau masalah yang mungkin muncul saat menangani bidirectional streaming di Rust gRPC, terutama pada aplikasi seperti chat?

Mengelola bidirectional streaming di Rust membutuhkan pemahaman mendalam tentang asynchronous programming. Tantangan utama adalah bagaimana menjaga agar komunikasi dua arah tetap sinkron tanpa deadlock. Selain itu, menjaga urutan pesan yang dikirim dan diterima juga menjadi tantangan, terutama dalam aplikasi chat. Kita juga harus memastikan bahwa koneksi tetap hidup meskipun tidak ada pesan untuk waktu tertentu. Jika client tiba-tiba disconnect, server harus mampu mendeteksi dan membersihkan resource terkait. Penggunaan channel seperti `mpsc` bisa menyebabkan kebocoran memori jika tidak dikelola dengan baik. Oleh karena itu, perencanaan desain dan pengujian menyeluruh sangat penting untuk menghindari bug sulit yang hanya muncul pada kondisi real-time.

---

## 4. Apa kelebihan dan kekurangan menggunakan `tokio_stream::wrappers::ReceiverStream` untuk streaming respons di layanan Rust gRPC?

Kelebihan utama `ReceiverStream` adalah kemampuannya untuk mengubah channel async `mpsc` menjadi stream yang dapat dikonsumsi oleh gRPC server. Ini sangat berguna karena banyak arsitektur aplikasi membutuhkan komunikasi antar task dalam bentuk channel. Integrasi dengan Tokio juga membuatnya efisien untuk digunakan dalam ekosistem asynchronous Rust. Namun, `ReceiverStream` memiliki kekurangan dalam hal fleksibilitas dan debugging. Misalnya, jika receiver sudah drop tapi sender masih aktif, kita tidak mendapat warning jelas. Selain itu, debugging alur streaming asinkron bisa sangat membingungkan karena error tidak selalu muncul langsung. Meskipun begitu, `ReceiverStream` tetap merupakan solusi praktis untuk sebagian besar kasus penggunaan server streaming.

---

## 5. Bagaimana kode gRPC Rust dapat disusun agar mendukung penggunaan ulang dan modularitas, serta memudahkan pemeliharaan dan pengembangan?

Struktur proyek yang baik dimulai dari pemisahan modul berdasarkan layanan, misalnya `payment`, `transaction`, dan `chat`. Setiap layanan bisa diletakkan dalam file atau direktori terpisah lengkap dengan logika, handler, dan service trait-nya. File `.proto` sebaiknya disimpan dalam direktori `proto/` agar mudah dikelola dan dikompilasi otomatis. Pendekatan ini tidak hanya meningkatkan keterbacaan kode, tapi juga memudahkan tim besar untuk bekerja secara paralel. Kita juga bisa menggunakan trait abstraction dan dependency injection untuk mempermudah testing. Dengan demikian, penambahan fitur baru atau penggantian komponen tertentu tidak memerlukan perubahan besar pada kode lainnya. Struktur modular seperti ini akan sangat membantu dalam jangka panjang saat proyek tumbuh besar dan kompleks.

---

## 6. Dalam implementasi `MyPaymentService`, langkah tambahan apa yang mungkin dibutuhkan untuk menangani logika pemrosesan pembayaran yang lebih kompleks?

Implementasi dasar `MyPaymentService` hanya mengembalikan respons dummy, tetapi sistem nyata memerlukan banyak tambahan. Pertama, validasi data input seperti format user ID dan nilai transaksi sangat penting. Kedua, integrasi dengan payment gateway eksternal seperti Midtrans, Xendit, atau Stripe harus dilakukan dengan aman dan andal. Kita juga perlu mencatat setiap transaksi ke database untuk keperluan pelacakan dan audit. Proses harus mampu menangani error seperti koneksi gagal atau transaksi ditolak oleh pihak ketiga. Status transaksi (pending, success, failed) juga perlu dikembalikan secara real-time. Jika diperlukan, retry mechanism dan idempotency key harus ditambahkan untuk mencegah transaksi ganda akibat gangguan jaringan.

---

## 7. Apa dampak adopsi gRPC terhadap arsitektur sistem terdistribusi, terutama dalam hal interoperabilitas dengan teknologi dan platform lain?

Adopsi gRPC membawa perubahan besar dalam cara layanan berkomunikasi di sistem terdistribusi. Dengan menggunakan Protocol Buffers, komunikasi menjadi lebih cepat dan hemat bandwidth dibandingkan JSON. Namun, interoperabilitas bisa menjadi tantangan karena semua layanan harus menggunakan `.proto` yang sama dan dikompilasi ulang dalam bahasa masing-masing. Di sisi positif, gRPC memungkinkan kita membangun layanan poliglot (multi-bahasa) dengan definisi kontrak yang ketat dan konsisten. Namun hal ini juga membuat onboarding developer baru atau integrasi dengan sistem lama jadi lebih rumit. Kita juga perlu mempertimbangkan tool support, karena debugging gRPC lebih sulit dibandingkan REST biasa. Meski begitu, gRPC memberikan fondasi kuat untuk sistem microservices modern dengan performa tinggi.

---

## 8. Apa kelebihan dan kekurangan HTTP/2 sebagai protokol dasar gRPC dibandingkan HTTP/1.1 atau HTTP/1.1 + WebSocket?

HTTP/2 memiliki banyak keunggulan teknis dibanding HTTP/1.1, salah satunya adalah multiplexing—kemampuan mengirim banyak request dalam satu koneksi. Ini sangat berguna untuk performa karena tidak perlu membuka koneksi baru setiap kali ada permintaan. Header compression juga membantu mengurangi overhead jaringan. Namun, implementasi HTTP/2 memerlukan dukungan penuh dari jaringan dan proxy, yang kadang menjadi hambatan di lingkungan enterprise. Dibandingkan WebSocket, gRPC memiliki struktur yang lebih formal dan mudah didokumentasikan. Namun WebSocket masih lebih fleksibel dalam beberapa kasus real-time yang tidak terlalu strict terhadap skema. Secara umum, HTTP/2 cocok untuk aplikasi skala besar yang membutuhkan efisiensi dan reliability.

---

## 9. Bagaimana model request-response dari REST API berbeda dengan kemampuan bidirectional streaming dari gRPC dalam hal komunikasi real-time?

REST API bersifat stateless dan synchronous: client mengirim permintaan dan menunggu satu respons sebelum lanjut ke permintaan berikutnya. Ini menyebabkan latensi lebih tinggi dalam aplikasi real-time seperti chat atau notifikasi langsung. Sebaliknya, gRPC bidirectional streaming memungkinkan komunikasi simultan dua arah tanpa blocking. Hal ini membuat interaksi lebih responsif dan efisien. Contohnya, dalam aplikasi chat, user bisa terus mengirim dan menerima pesan tanpa harus menunggu server menyelesaikan respons sebelumnya. Ini menciptakan pengalaman pengguna yang lebih natural dan real-time. Perbedaan fundamental ini membuat gRPC lebih unggul dalam banyak skenario komunikasi interaktif.

---

## 10. Apa implikasi pendekatan berbasis skema pada gRPC (menggunakan Protocol Buffers) dibanding pendekatan tanpa skema seperti JSON pada REST API?

Pendekatan berbasis skema seperti Protocol Buffers memberi jaminan bahwa data yang dikirim dan diterima sudah tervalidasi secara struktur dan tipe. Ini meningkatkan keandalan komunikasi antar layanan, karena kesalahan dapat dideteksi sejak proses serialisasi. Namun, fleksibilitasnya lebih rendah dibanding JSON karena setiap perubahan skema harus disertai dengan update file `.proto` dan rekode ulang client. JSON lebih fleksibel dan mudah diinspeksi, tetapi bisa menyebabkan kesalahan jika struktur data tidak disepakati bersama. Penggunaan Protocol Buffers juga mempercepat parsing dan mengurangi ukuran payload. Tetapi debugging lebih sulit tanpa alat bantu tambahan. Secara keseluruhan, schema-based cocok untuk sistem berskala besar dan terstandarisasi, sedangkan JSON lebih cocok untuk prototipe atau integrasi cepat.

---
