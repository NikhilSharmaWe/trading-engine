use std::{collections::HashMap, fmt::format};
use rust_decimal::Decimal;
use super::orderbook::{OrderBook, Order};


#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TradingPair {
	base: String,
	quote: String,
}

impl TradingPair {
	pub fn new(base: String, quote: String) -> TradingPair {
		TradingPair{
			base,
			quote,
		}	
	}

	pub fn to_string(self) -> String {
		format!("{}_{}", self.base, self.quote)
	}
}

pub struct MatchingEngine {
	orderbooks: HashMap<TradingPair, OrderBook>,
}

impl MatchingEngine {
	pub fn new() -> MatchingEngine {
		MatchingEngine{
			orderbooks: HashMap::new(),
		}
	}

	pub fn add_new_market(&mut self, pair: TradingPair) {
		self.orderbooks.insert(pair.clone(), OrderBook::new());
		println!("opening a new orderbook for market {:?}", pair.to_string());
	}

	pub fn place_limit_order(&mut self, pair: TradingPair, price: Decimal, order: Order) -> Result<(), String>{
		match self.orderbooks.get_mut(&pair) { 
			Some(orderbook) => {
				orderbook.add_limit_order(price, order);
				println!("placed limit order at price level {}", price);
				Ok(())
			}
			None => {
				Err(format!(
					"the orderbook for the given trading pair ({}) does not exist", 
					pair.to_string()
				))
			}
		}
	}
}