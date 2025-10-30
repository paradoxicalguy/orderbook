use std::cmp::Ordering;
use std::io;
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq)]
enum OrderType {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
struct Order {
    id: u64,
    order_type: OrderType,
    price: f64,
    quantity: f64,
}

#[derive(Debug, Clone)]
struct Trade {
    buy_id: u64,
    sell_id: u64,
    price: f64,
    quantity: f64,
}

#[derive(Debug)]
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
        self.sort_books();
        self.match_orders();
    }

    fn sort_books(&mut self) {
        self.buy_orders
            .sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap_or(Ordering::Equal));
        self.sell_orders
            .sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(Ordering::Equal));
    }

    fn match_orders(&mut self) {
        let mut trades = Vec::new();
        let mut i = 0;

        while i < self.buy_orders.len() {
            let mut j = 0;
            while j < self.sell_orders.len() {
                let buy = &mut self.buy_orders[i];
                let sell = &mut self.sell_orders[j];

                if buy.price >= sell.price {
                    let qty = buy.quantity.min(sell.quantity);
                    let trade_price = sell.price;

                    trades.push(Trade {
                        buy_id: buy.id,
                        sell_id: sell.id,
                        price: trade_price,
                        quantity: qty,
                    });

                    buy.quantity -= qty;
                    sell.quantity -= qty;

                    if sell.quantity == 0.0 {
                        self.sell_orders.remove(j);
                    } else {
                        j += 1;
                    }

                    if buy.quantity == 0.0 {
                        break;
                    }
                } else {
                    j += 1;
                }
            }

            if self.buy_orders[i].quantity == 0.0 {
                self.buy_orders.remove(i);
            } else {
                i += 1;
            }
        }

        self.trades.extend(trades);
    }

    fn show_book(&self) {
        println!("\n===== order-book =====");
        println!("--- buy orders ---");
        for o in &self.buy_orders {
            println!(
                "Buy #{:<3} | Price: {:<6} | Qty: {:<5}",
                o.id, o.price, o.quantity
            );
        }

        println!("--- sell orders ---");
        for o in &self.sell_orders {
            println!(
                "Sell #{:<3} | Price: {:<6} | Qty: {:<5}",
                o.id, o.price, o.quantity
            );
        }
    }

    fn show_trades(&self) {
        println!("\n===== trades executed =====");
        for t in &self.trades {
            println!(
                "Buy #{} matched with Sell #{} | Price: {:<6} | Qty: {:<5}",
                t.buy_id, t.sell_id, t.price, t.quantity
            );
        }
    }
}

fn main() {
    let mut book = OrderBook::new();
    let mut next_id = 1;

    println!("📘 simple orderbook cli");
    println!("commands:");
    println!("  add buy <price> <qty>");
    println!("  add sell <price> <qty>");
    println!("  book          - show current order book");
    println!("  trades        - show trade history");
    println!("  exit          - quit the program");

    loop {
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("exiting...");
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "add" => {
                if parts.len() != 4 {
                    println!("Usage: add <buy/sell> <price> <quantity>");
                    continue;
                }

                let order_type = match parts[1].to_lowercase().as_str() {
                    "buy" => OrderType::Buy,
                    "sell" => OrderType::Sell,
                    _ => {
                        println!("invalid order type: {}", parts[1]);
                        continue;
                    }
                };

                let price: f64 = match parts[2].parse() {
                    Ok(p) => p,
                    Err(_) => {
                        println!("invalid price");
                        continue;
                    }
                };

                let qty: f64 = match parts[3].parse() {
                    Ok(q) => q,
                    Err(_) => {
                        println!("invalid quantity");
                        continue;
                    }
                };

                book.add_order(Order {
                    id: next_id,
                    order_type,
                    price,
                    quantity: qty,
                });
                next_id += 1;

                println!("order added.");
            }

            "book" => {
                book.show_book();
            }

            "trades" => {
                book.show_trades();
            }

            _ => {
                println!("unknown command. try: add / book / trades / exit");
            }
        }
    }
}
