use std::cell::{RefCell, Cell};

use super::TradeItem;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum State {
    Init,
    Pending,
    Complete,
}

pub struct Order {
    pub state: State,
    pub item: Box<dyn TradeItem>,
    pub amount: u32,
    pub modifiers: Vec<f32>,
}

impl Order {
    pub fn new(item: Box<dyn TradeItem>, amount: u32, modifiers: Vec<f32>) -> Self {
        Self {
            state: State::Init,
            item,
            amount,
            modifiers,
        }
    }

    pub fn pending(&self) -> u32 {
        self.item.price(self.modifiers.clone()) * self.amount
    }

    fn valid_item_amount(&self, amount: u32) -> (u32, u32) {
        if amount >= self.pending() {
            return (amount / self.item.price(self.modifiers), amount - self.pending())
        }
        (self.amount, 0)
    }

    pub fn update(&mut self, amount: u32) -> (State, u32) {
        let (deduct, returns) = self.valid_item_amount(amount);
        self.amount -= deduct;

        if self.amount <= 0 {
            self.state = State::Complete;
        } else {
            self.state = State::Pending;
        }

        (self.state.clone(), returns)
    }
}


pub struct Transaction {
    pub state: State,
    pub orders: Vec<RefCell<Order>>,
    pub refund: u32,
}

impl Transaction {
    pub fn new() -> Self {
        Self {
            state: State::Init,
            orders: Vec::new(),
            refund: 0,
        }
    }

    pub fn add_order(&mut self, order: RefCell<Order>) {
        self.orders.push(order);
    }

    pub fn execute(&mut self, amount: u32) -> State {
        let mut remaining = amount;

        let mut uncomplete = self.orders
            .iter_mut()
            .filter(|item| item.borrow().state != State::Complete);
            

        if uncomplete.count() == 0 {
            return State::Complete
        }

        loop {
            if remaining <= 0 {
                break;
            }

            match uncomplete.next() {
                Some(order) => {
                    let (_, amnt) = order.borrow_mut().update(remaining); 
                    remaining -= amnt;
                },
                None => {
                    break
                },   
            } 
        }

        if self.orders.iter_mut().all(|item| item.get_mut().state == State::Complete) {
            self.state = State::Complete;
        }



        self.state
    }
}


