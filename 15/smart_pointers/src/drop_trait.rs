struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop() {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        println!("CustomSmartPointer created.");
        drop(c);
        println!("CustomSmartPointer dropped before the end of the block.");
    }
}
