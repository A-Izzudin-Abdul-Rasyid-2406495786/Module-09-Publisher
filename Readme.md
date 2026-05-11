## Tutorial 8: Event-Driven Architecture - Publisher

### a. How much data your publisher program will send to the message broker in one run?
Program *publisher* akan mengirimkan **5 buah pesan (event)** ke *message broker* dalam satu kali eksekusi (satu kali `cargo run`). Kelima data tersebut merepresentasikan event `user_created` untuk 5 user yang berbeda secara berurutan (yaitu data untuk user: Amir, Budi, Cica, Dira, dan Emir).

### b. The url of: “amqp://guest:guest@localhost:5672” is the same as in the subscriber program, what does it mean?
URL yang sama persis ini menandakan bahwa program *publisher* dan *subscriber* **terhubung ke *server message broker* (RabbitMQ) yang sama**. 

Dalam arsitektur *event-driven*, agar *subscriber* dapat menerima pesan yang dikirimkan oleh *publisher*, keduanya harus berkomunikasi melalui "perantara" yang sama. Oleh karena itu, *publisher* mengirim pesan ke alamat `localhost:5672` (dengan kredensial `guest`), dan *subscriber* juga "mendengarkan" (*listen*) pada alamat dan kredensial yang persis sama untuk mengambil pesan-pesan tersebut dari antrian.