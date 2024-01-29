// Créez un module pour gérer la simulation du flux TCP
pub mod tcpstream_mock {
    use std::io::{Read, Write};

    pub struct TcpStreamMock {
        // Utilisez un vecteur pour stocker les données simulées
        data: Vec<u8>,
    }

    impl TcpStreamMock {
        // Initialisez un nouveau TcpStreamMock
        pub fn new() -> Self {
            TcpStreamMock { data: Vec::new() }
        }

        // Écrivez une chaîne dans le TcpStreamMock
        pub fn write_str(&mut self, s: &str) {
            self.data.extend_from_slice(s.as_bytes());
        }

        // Lisez la prochaine chaîne du TcpStreamMock
        pub fn read_string(&mut self) -> Option<String> {
            let mut buffer = Vec::new();
            while let Some(&byte) = self.data.first() {
                if byte == b'\n' {
                    break;
                }
                buffer.push(byte);
                self.data = &self.data[1..];
            }
            self.data = &self.data[1..]; // Ignore the newline
            String::from_utf8(buffer).ok()
        }
    }

    // Implémentez Read et Write pour TcpStreamMock
    impl Read for TcpStreamMock {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let len = std::cmp::min(buf.len(), self.data.len());
            buf[..len].copy_from_slice(&self.data[..len]);
            self.data = &self.data[len..];
            Ok(len)
        }
    }

    impl Write for TcpStreamMock {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
}