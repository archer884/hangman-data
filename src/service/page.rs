pub struct Page {
    page: i32,
    size: i32,
}

impl Page {
    pub fn new(page: i32) -> Page {
        Page::with_size(page, 10)
    }

    pub fn with_size(page: i32, size: i32) -> Page {
        Page {
            page: page,
            size: size,
        }
    }

    pub fn skip(&self) -> i32 {
        self.size * (self.page - 1)
    }

    pub fn take(&self) -> i32 {
        self.size
    }
}