/*
 * Copyright 2026 Nicolas Spijkerman
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::{Order, OrderType};
use ordered_float::OrderedFloat;
use std::collections::{BTreeMap, VecDeque};

/// Queue of resting orders at that price
type PriceLevel = BTreeMap<OrderedFloat<f64>, VecDeque<Order>>;

pub struct OrderBook {
    pub symbol: String,
    pub bids: PriceLevel,
    pub asks: PriceLevel,
}

impl OrderBook {
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            bids: PriceLevel::new(),
            asks: PriceLevel::new(),
        }
    }

    pub fn submit(&mut self, order: Order) {
        match order.order_type {
            OrderType::Market => {}
            OrderType::Limit { price: _ } => {}
            OrderType::Stop { price_trigger: _ } => {}
            OrderType::StopLimit {
                price_trigger: _,
                price_limit: _,
            } => {}
        }
    }
}
