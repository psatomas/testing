#[test]
fn museum_with_impressive_art_collection_charges_more_for_admission() {
    let mut museum = Museum::new();
    museum.buy_painting("Mona Lisa");
    museum.buy_painting("The Starry Night");
    museum.buy_painting("Girl with a Pearl Earring");

    museum.sell_ticket();

    assert_eq!(museum.revenue, 35);
}

impl TicketSeller for Museum {
    fn sell_ticket(&mut self) {
        if self.has_impressive_collection() {
            self.revenue += 35;
        } else {
            self.revenue += 25;
        }
    }
}

// tests/venue_management_test.rs
#[rstest]
fn venue_management_interacts_with_museum_venue(museum_with_three_paintings: Museum) {
    let mut venue_mgmt = VenueManagement::new(museum_with_three_paintings);
    venue_mgmt.make_money();

    assert_eq!(venue_mgmt.venue.paintings.len(), 3);
    assert_eq!(venue_mgmt.venue.revenue, 35);
}