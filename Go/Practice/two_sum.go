package main

// Question 1 from leetcode

func twoSum(nums []int, target int) []int {
	needPosition := make(map[int]int)
	for i, num := range nums {
		pos, ok := needPosition[num]
		if ok {
			return []int{pos, i}
		}
		needPosition[target-num] = i
	}
	return nil
}
