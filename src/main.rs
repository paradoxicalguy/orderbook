use std::cmp::Ordering;

struct Order {
    id: u64,
    price: f64,
    quantity: f64,
    order_type: OrderType,
}

enum OrderType {
    Buy,
    Sell,
}

struct Trade {
    buy_id: u64,
    sell_id: u64,
    price: f64,
    quantity: f64,
}

struct OrderBook {
    buy_orders: Vec<Order>,
    sell_orders: Vec<Order>,
    trades: Vec<Trade>,
}

impl OrderBook {
    fn new() -> Self {
        Self {
            buy_orders: Vec::new(),
            sell_orders: Vec::new(),
            trades: Vec::new(),
        }
    }

    fn add_order(&mut self, order: Order) {
        match order.order_type {
            OrderType::Buy => self.buy_orders.push(order),
            OrderType::Sell => self.sell_orders.push(order),
        }
    }

    fn sort_books(&mut self) {
        self.buy_orders
            .sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap_or(Ordering::Equal));
        self.sell_orders
            .sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(Ordering::Equal));
    }

    fn match_orders(&mut self) {
        self.sort_books();
        let mut trades = Vec::new();

        let mut i = 0;
        while i < self.buy_orders.len() {
            let mut j = 0;
            while j < self.sell_orders.len() {
                let (buy_price, buy_qty, buy_id) = {
                    let buy = &self.buy_orders[i];
                    (buy.price, buy.quantity, buy.id)
                };
                let (sell_price, sell_qty, sell_id) = {
                    let sell = &self.sell_orders[j];
                    (sell.price, sell.quantity, sell.id)
                };

                if buy_price >= sell_price {
                    let qty = buy_qty.min(sell_qty);
                    let trade_price = sell_price;

                    trades.push(Trade {
                        buy_id,
                        sell_id,
                        price: trade_price,
                        quantity: qty,
                    });

                    self.buy_orders[i].quantity -= qty;
                    self.sell_orders[j].quantity -= qty;

                    if self.sell_orders[j].quantity == 0.0 {
                        self.sell_orders.remove(j);
                    } else {
                        j += 1;
                    }

                    if self.buy_orders[i].quantity == 0.0 {
                        break;
                    }
                } else {
                    j += 1;
                }
            }

            if i < self.buy_orders.len() && self.buy_orders[i].quantity == 0.0 {
                self.buy_orders.remove(i);
            } else {
                i += 1;
            }

            if self.buy_orders.is_empty() || self.sell_orders.is_empty() {
                break;
            }
        }

        self.trades.extend(trades);
    }

    fn show_book(&self) {
        println!("== order book ==");
        println!("-- buy orders --");
        for o in &self.buy_orders {
            println!("buy #{} | price: {} | qty: {}", o.id, o.price, o.quantity);
        }

        println!("-- sell orders --");
        for o in &self.sell_orders {
            println!("sell #{} | price: {} | qty: {}", o.id, o.price, o.quantity);
        }
    }

    fn show_trades(&self) {
        println!("== trades executed ==");
        for t in &self.trades {
            println!(
                "buy #{} matches with sell #{} | price: {} | qty: {}",
                t.buy_id, t.sell_id, t.price, t.quantity
            );
        }
    }
}

fn main() {
    let mut book = OrderBook::new();
    println!("new order book created");

    book.add_order(Order {
        id: 1,
        price: 100.0,
        quantity: 3.0,
        order_type: OrderType::Buy,
    });
    book.add_order(Order {
        id: 2,
        price: 99.0,
        quantity: 2.0,
        order_type: OrderType::Sell,
    });
    book.add_order(Order {
        id: 3,
        price: 102.0,
        quantity: 1.0,
        order_type: OrderType::Buy,
    });
    book.add_order(Order {
        id: 4,
        price: 101.0,
        quantity: 4.0,
        order_type: OrderType::Sell,
    });

    book.match_orders();

    book.show_book();
    book.show_trades();
}
