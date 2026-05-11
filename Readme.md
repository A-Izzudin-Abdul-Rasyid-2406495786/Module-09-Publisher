## Modul 9: Event-Driven Architecture - Publisher

### a. How much data your publisher program will send to the message broker in one run?
Program *publisher* akan mengirimkan **5 buah pesan (event)** ke *message broker* dalam satu kali eksekusi (satu kali `cargo run`). Kelima data tersebut merepresentasikan event `user_created` untuk 5 user yang berbeda secara berurutan (yaitu data untuk user: Amir, Budi, Cica, Dira, dan Emir).

### b. The url of: “amqp://guest:guest@localhost:5672” is the same as in the subscriber program, what does it mean?
URL yang sama persis ini menandakan bahwa program *publisher* dan *subscriber* **terhubung ke *server message broker* (RabbitMQ) yang sama**. 

Dalam arsitektur *event-driven*, agar *subscriber* dapat menerima pesan yang dikirimkan oleh *publisher*, keduanya harus berkomunikasi melalui "perantara" yang sama. Oleh karena itu, *publisher* mengirim pesan ke alamat `localhost:5672` (dengan kredensial `guest`), dan *subscriber* juga "mendengarkan" (*listen*) pada alamat dan kredensial yang persis sama untuk mengambil pesan-pesan tersebut dari antrian.

## Running RabbitMQ as message broker
![rabbitMQ](rabbitMQ.png)

## Sending and processing event
![processingevent](processingevent.png)

## Monitoring Chart Based on Publisher

![RabbitMQ Message Rates Chart](spike.png)

**Penjelasan Lonjakan (Spike) pada Grafik:**

Lonjakan (*spike*) yang terlihat pada grafik "Message rates" di atas berhubungan langsung dengan eksekusi program *publisher*. 

Setiap kali program *publisher* dijalankan (`cargo run`), ia akan mengirimkan *burst* atau rentetan 5 pesan sekaligus secara instan ke *message broker* (RabbitMQ). Ketika saya menjalankan program *publisher* berulang-ulang secara cepat dalam waktu singkat, RabbitMQ menerima volume pesan masuk (pengiriman/ *publish*) yang sangat tinggi secara tiba-tiba dalam satu detik. 

Sistem RabbitMQ merespons masuknya lalu lintas data yang mendadak ini dengan menampilkannya sebagai lonjakan tajam pada grafik *Message rates*. Jika *subscriber* sedang menyala, grafik pengiriman keluar (*deliver* / pengambilan pesan) juga akan ikut melonjak tajam karena *subscriber* dengan cepat memproses tumpukan pesan yang baru saja masuk tersebut.


## Bonus: Make it works on cloud

Pada bagian bonus ini, saya tidak lagi menggunakan `localhost` sebagai *message broker*, melainkan beralih menggunakan layanan *cloud* agar sistem ini bersifat terdistribusi secara nyata.

### Cloud Service yang Digunakan
Saya menggunakan **CloudAMQP**, sebuah layanan *managed* untuk RabbitMQ dan LavinMQ. Instance yang saya gunakan berjalan di atas *cluster* **LavinMQ** (versi RabbitMQ yang lebih ringan dan efisien) dengan nama instance `jilkwcrc` yang berlokasi di server `armadillo.rmq.cloudamqp.com`.

### Cara Menggunakan Cloud Broker
1. **Konfigurasi Instance:** Saya membuat instance gratis (Little Lemur) di CloudAMQP untuk mendapatkan URL koneksi AMQPS yang aman.
2. **Pembaruan Kode:** Saya mengubah alamat koneksi pada file `src/main.rs` baik di sisi *publisher* maupun *subscriber* dari `"amqp://guest:guest@localhost:5672"` menjadi URL *secure* dari CloudAMQP: `amqps://jilkwcrc:3B3qb2ONEYepKlWPrYqLcOVHrCXrz9S6@armadillo.rmq.cloudamqp.com/jilkwcrc`.
3. **Eksekusi Lokal ke Cloud:** Program tetap dijalankan di komputer lokal saya (Rasyid's Computer), namun pengiriman dan penerimaan pesan dilakukan melalui internet ke server CloudAMQP.

### Bukti Eksperimen di Cloud

![Cloud Management Dashboard](cloudservice.png)
*Gambar di atas menunjukkan dashboard LavinMQ di CloudAMQP yang memantau trafik pesan secara real-time di server cloud.*

![Subscriber received message from Cloud](cloudterminal.png)
*Gambar di atas menunjukkan terminal subscriber pada komputer saya (NPM: 2406495786) yang berhasil menerima pesan dari broker yang berada di cloud.*

### Refleksi
Dengan menjalankan *broker* di *cloud*, aplikasi *publisher* dan *subscriber* tidak harus berada di satu mesin yang sama. Hal ini memungkinkan skalabilitas yang lebih tinggi di mana banyak *subscriber* dari berbagai lokasi berbeda dapat terhubung ke satu *queue* yang sama di internet untuk memproses beban kerja secara bersama-sama.