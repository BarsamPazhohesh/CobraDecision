public class Solution {
    public int[] TwoSum(int[] numbers, int target) {
		int left = 0;
		int right = numbers.Length-1;
		while(left<right)
		{
			int sum = numbers[left] + numbers[right];
			if(sum<target)
				left++;
			else if(sum>target)
				right--;
			else
				return new int[2]{++left,++right};
		}
		return new int[2]{-1,-1};
    }
}