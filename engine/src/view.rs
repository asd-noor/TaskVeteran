#[derive(PartialEq)]
pub struct View {
    name: String,
}

impl View {
    pub fn new(name: String) -> Self {
        View { name: name }
    }
}

#[cfg(test)]
mod tests;
