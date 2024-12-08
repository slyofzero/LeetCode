pub fn max_profit_1(prices: Vec<i32>) -> i32 {
    let mut min_price = i32::MAX;
    let mut max_profit = 0;

    for price in prices {
        if price < min_price {
            min_price = price;
        } else if price - min_price > max_profit {
            max_profit = price - min_price;
        }
    }

    max_profit
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 {return 0;}

    let mut profit = 0;

    for i in 1..prices.len() {
        if prices[i] > prices[i - 1] {
            profit += prices[i] - prices[i - 1];
        }
    }

    profit
}

fn main() {
    let prices = Vec::from([7,1,2,3,4,5,3,6,4]);
    let profit = max_profit(prices);
    println!("{}", profit);
}