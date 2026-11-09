pub enum OrderStatus {
    Pending,
    Paid,
    Sent,
}
pub fn process_order(status: OrderStatus) {
    match status {
        OrderStatus::Pending => {
            println!("Order is pending")
        }
        OrderStatus::Paid => {
            println!("Order is paid")
        }
        OrderStatus::Sent => {
            println!("Order is sent")
        }
    }
}

pub fn run_tests() {
    process_order(OrderStatus::Pending);
    process_order(OrderStatus::Paid);
    process_order(OrderStatus::Sent);
}
