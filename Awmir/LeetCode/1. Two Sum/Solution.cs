public static int[] TwoSum(int[] nums, int target)
{
    Dictionary<int, int> seen = new Dictionary<int, int>();
    int end = nums.Length;
    for (int i = 0; i < end; i++)
    {
        int complement = target - nums[i];

        if (seen.ContainsKey(complement))
        {
            return new int[] { seen[complement], i };
        }
        seen[nums[i]] = i;
    }
    return null;
}