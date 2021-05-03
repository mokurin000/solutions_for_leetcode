impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut lowest = prices[0];
        let mut highest = prices[0];
        let mut profit_past = 0;
        let mut profit_curr = 0;
    
        for i in 1..prices.len() {
            if prices[i] >= highest { // 涨
                highest = prices[i];
                profit_curr = highest - lowest;
            } else { // 跌了
                lowest = prices[i];
                highest = prices[i];
                profit_past += profit_curr;
                profit_curr = 0;
            }
        }
    
        profit_curr + profit_past
    }
}
