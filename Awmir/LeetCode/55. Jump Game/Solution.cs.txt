public class Solution {
    public bool CanJump(int[] nums) {
        int maxStep = 0;
        int end = nums.Length;
        for(int i = 0;i<end;i++)
        {
            if(i>maxStep) return false;
            maxStep=Math.Max(maxStep,i+nums[i]);
            if(maxStep>=end-1) return true;
        }
        return false;
    }
}