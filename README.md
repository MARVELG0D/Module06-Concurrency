# Module06-Concurrency

## Commit 1 Reflection Notes

Dalam fungsi `handle_connection`, saya menggunakan `BufReader` untuk membaca data dari `TcpStream` secara efisien dan membaginya menjadi baris-baris teks. Poin krusial di sini adalah penggunaan `.take_while(|line| !line.is_empty())`, yang berfungsi untuk menangkap seluruh *HTTP header* dan berhenti tepat saat server menemui baris kosong yang menandakan akhir dari sebuah *HTTP request*. Proses ini memungkinkan saya untuk melihat bagaimana browser berkomunikasi secara mentah dengan server, mulai dari *Request Line* (seperti `GET / HTTP/1.1`) hingga berbagai informasi *headers* lainnya sebelum nantinya server dapat mengirimkan respons balik.