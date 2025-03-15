public class Solution {
    public int RemoveDuplicates(int[] nums) {
        int end = nums.Length;
        if (end == 0) return 0;
        
        int k = 1; 
        for (int i = 1; i < end; i++) {
            if (nums[i] != nums[i - 1]) 
            { 
                nums[k] = nums[i]; 
                k++;
            }
        }
        return k;
    }
}
