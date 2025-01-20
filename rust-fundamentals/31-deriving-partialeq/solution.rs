#[derive(Debug, PartialEq)]
pub enum OrderStatus {
    Pending,
    Shipped,
    Cancelled(String),
}

// Example use case
pub fn main() {
    let status1 = OrderStatus::Pending;
    let status2 = OrderStatus::Pending;
    assert_eq!(status1, status2);

    let cancelled1 = OrderStatus::Cancelled("Out of stock".to_string());
    let cancelled2 = OrderStatus::Cancelled("Out of stock".to_string());
    assert_eq!(cancelled1, cancelled2);

    assert_ne!(status1, cancelled1);

    let cancelled3 = OrderStatus::Cancelled("Customer request".to_string());
    assert_ne!(cancelled1, cancelled3);

    assert_ne!(cancelled2, cancelled3);
}

