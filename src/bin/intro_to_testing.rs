// lib.rs
#[derive(Debug)]
struct Museum {
    paintings: Vec<String>,
    revenue: u32,
}

impl Museum {
    fn new() -> Self {
        Self {
            paintings: vec![],
            revenue: 0,
        }
    }

    fn buy_painting(&mut self, painting: &str) {
        self.paintings.push(painting.to_string());
    }

    fn sell_ticket(&mut self) {
        self.revenue += 25;
    }

    fn has_impressive_collection(&self) -> bool {
        self.paintings.len() > 2
    }
}