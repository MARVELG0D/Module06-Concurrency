# Module06-Concurrency

## Commit 1 Reflection Notes

Dalam fungsi `handle_connection`, saya menggunakan `BufReader` untuk membaca data dari `TcpStream` secara efisien dan membaginya menjadi baris-baris teks. Poin krusial di sini adalah penggunaan `.take_while(|line| !line.is_empty())`, yang berfungsi untuk menangkap seluruh *HTTP header* dan berhenti tepat saat server menemui baris kosong yang menandakan akhir dari sebuah *HTTP request*. Proses ini memungkinkan saya untuk melihat bagaimana browser berkomunikasi secara mentah dengan server, mulai dari *Request Line* (seperti `GET / HTTP/1.1`) hingga berbagai informasi *headers* lainnya sebelum nantinya server dapat mengirimkan respons balik.

## Commit 2 Reflection Notes

Pada Milestone 2, saya belajar cara mengirimkan respons HTTP yang valid agar browser dapat menampilkan konten visual. Fungsi `handle_connection` kini tidak hanya membaca *request*, tetapi juga menyusun respons yang terdiri dari **status line** (`HTTP/1.1 200 OK`), **headers** (khususnya `Content-Length` untuk memberi tahu browser ukuran data yang dikirim), dan **response body** yang berisi konten file `hello.html`. Penggunaan `fs::read_to_string` memudahkan pemisahan logika kode Rust dengan konten HTML, sementara `stream.write_all` memastikan seluruh paket data tersebut terkirim kembali ke klien melalui koneksi TCP yang terbuka.
![Commit 2 screen capture](image.png)

## Commit 3 Reflection Notes

Pada Milestone 3, Pak Ade ingin mengajarkan cara membangun mekanisme *routing* sederhana untuk memvalidasi *request path* dan memberikan respons yang sesuai, seperti mengembalikan halaman `404 Not Found` jika rute tidak dikenali. Selain logika percabangan, poin pembelajaran krusial di sini adalah pentingnya melakukan *refactoring*. Tanpa *refactoring*, blok `if-else` akan memunculkan banyak duplikasi kode untuk proses membaca file dan menyusun respons. Dengan melakukan *refactoring*, kita cukup membedakan deklarasi `status_line` dan nama file di dalam blok kondisi, lalu menyatukan eksekusi pembacaan file dan pengiriman data di akhir. Hal ini membuat struktur kode terhindar dari redundansi (menerapkan prinsip *Don't Repeat Yourself*), lebih bersih, dan lebih mudah dikembangkan ke depannya.
![Commit 3](image-1.png)