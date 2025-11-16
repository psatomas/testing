impl Museum {
    const MAXIMUM_CAPACITY: usize = 3;

    /// Creates a new Museum instance.
    ///
    /// # Examples
    /// ```
    /// use testing::attractions::Museum;
    ///
    /// let museum = Museum::new();
    /// let empty_vec: Vec<String> = Vec::new();
    /// assert_eq!(museum.paintings, empty_vec);
    /// assert_eq!(museum.revenue, 0);
    /// ```
    pub fn new() -> Self {
        Self {
            paintings: vec![],
            revenue: 0,
        }
    }

    /// Buys a painting for the museum.
    ///
    /// # Examples
    /// ```
    /// use testing::attractions::Museum;
    ///
    /// let mut museum = Museum::new();
    /// museum.buy_painting("Mona Lisa");
    /// assert_eq!(museum.paintings, vec!["Mona Lisa".to_string()])
    /// ```
    pub fn buy_painting(&mut self, painting: &str) {
        if self.paintings.len() >= Self::MAXIMUM_CAPACITY {
            panic!("Museum does not have storage space for another painting!");
        }

        self.paintings.push(painting.to_string());
    }
}