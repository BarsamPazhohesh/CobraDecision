class Solve_1
{
    public List<List<int>> ThreeSum(int[] nums)
    {
        List<List<int>> result = new List<List<int>>();
        int end = nums.Length;
        for (int i = 0; i < end; i++)
        {
            for (int j = i + 1; j < end; j++)
            {
                for (int w = j + 1; w < end; w++)
                {
                    if (nums[i] + nums[j] + nums[w] == 0)
                    {
                        List<int> temp = new List<int> { nums[i], nums[j], nums[w] }
                        result.Add(temp);
                    }
                }
            }
        }
        var uniqueLists = result
            .Select(list => list.OrderBy(x => x).ToList())  // Sort each list
            .Distinct(new ListComparer()) // Remove Duplicates 
            .ToList();
        return uniqueLists;
    }

}
class ListComparer : IEqualityComparer<List<int>> // comparative class for List<int>
{
    public bool Equals(List<int> x, List<int> y)
    {
        return x.SequenceEqual(y);
    }

    public int GetHashCode(List<int> obj)
    {
        return obj.Aggregate(17, (hash, item) => hash * 31 + item.GetHashCode());

        // new List<int> { 1, 2, 3 }

        // Step 1: hash = 17
        // Step 2: hash = (17 * 31) + 1 = 528
        // Step 3: hash = (528 * 31) + 2 = 16370
        // Step 4: hash = (16370 * 31) + 3 = 507473

        // Final hash = 507473

    }
}