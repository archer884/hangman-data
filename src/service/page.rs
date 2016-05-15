pub struct Page {
    index: i64,
    size: i64,
}

impl Page {
    pub fn new(index: i64) -> Page {
        Page::with_size(index, 10)
    }

    pub fn with_size(index: i64, size: i64) -> Page {
        Page {
            index: index,
            size: size,
        }
    }

    pub fn skip(&self) -> i64 {
        self.index * self.size
    }

    pub fn take(&self) -> i64 {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::Page;
    
    #[test]
    fn page_0_10() {
        let page = Page::new(0);
        
        assert_eq!(0, page.skip());
        assert_eq!(10, page.take());
    }
}