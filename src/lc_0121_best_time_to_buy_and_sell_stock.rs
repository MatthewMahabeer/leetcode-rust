pub fn max_profit(prices: Vec<i32>) -> i32 {
    let (mut profit, mut buy) = (0, prices[0]);

    for i in 1..prices.len() {
        profit = i32::max(profit, prices[i] - buy);
        buy = i32::min(buy, prices[i]);
    }

    return profit;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }
}
