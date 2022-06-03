pub struct Server<T> {server: T}

impl<T> Server<T> where
{
    pub fn new(
        server: T,
    ) -> Self {
        Self {
            server,
        }
    }

    pub fn run(mut self) -> T {
        self.server
    }
}