package main

// Question 121 from leetcode

func badMaxProfit(prices []int) int {
	maxProfit := 0
	for i := range prices {
		for j := i + 1; j < len(prices); j += 1 {
			profit := prices[j] - prices[i]
			if maxProfit < profit {
				maxProfit = profit
			}
		}
	}
	return maxProfit
}

func maxProfit(prices []int) int {
	buyPrice := prices[0]
	profit := 0
	for _, price := range prices {
		if price < buyPrice {
			buyPrice = price
		} else if profit < price-buyPrice {
			profit = price - buyPrice
		}
	}
	return profit
}
