mod matching_engine;
use matching_engine::orderbook::{BidOrAsk, OrderBook, Order};
use matching_engine::engine::{TradingPair, MatchingEngine};
use rust_decimal_macros::dec;

fn main() {
    let buy_order_from_alice = Order::new(BidOrAsk::Bid, 5.5);
    let buy_order_from_bob = Order::new(BidOrAsk::Bid, 2.45);

    let mut orderbook = OrderBook:: new();
    orderbook.add_limit_order(dec!(4.4), buy_order_from_alice);
    orderbook.add_limit_order(dec!(4.4), buy_order_from_bob);
 
    let sell_order = Order::new(BidOrAsk::Ask, 6.5);
    orderbook.add_limit_order(dec!(20.0), sell_order);
    // println!("{:?}", orderbook);

    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    engine.add_new_market(pair.clone());

    let buy_order = Order::new(BidOrAsk::Bid, 6.5);
    // let res = engine.place_limit_order(pair, 10.000, buy_order);
    // match res {
    //     Ok(()) => {}
    //     Err(err) => {println!("ERROR: {}", err)}
    // }

    // let eth_pair = TradingPair::new("ETH".to_string(), "USD".to_string());
    engine.place_limit_order(pair, dec!(10.000), buy_order).unwrap();

    // let res = engine.place_limit_order(eth_pair, 10.000, buy_order);
    // match res {
    //     Ok(()) => {}
    //     Err(err) => {println!("ERROR: {}", err)}
    // }
}