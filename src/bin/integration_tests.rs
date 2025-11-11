/* tests/venue_management_test.rs */
use pretty_assertions::assert_eq;

use testing::attractions::Museum;
use testing::management::VenueManagement;

#[test]
fn venue_management_interacts_with_venue() {
    let mut museum = Museum::new();
    museum.buy_painting("Mona Lisa");

    let mut venue_mgmt = VenueManagement::new(museum);
    venue_mgmt.make_money();

    assert_eq!(venue_mgmt.venue.revenue, 25);
}