using System;
using System.Collections.Generic;
using System.Diagnostics;

class Program
{
    static void Main()
    {
        string input = "AAAABBB1111CCCDD";

        Stopwatch sw = new Stopwatch(); 
        sw.Start(); 

        Dictionary<char, int> SeqComp = new Dictionary<char, int>();
        for (int i = 0; i < input.Length; i++)
        {
            if (!SeqComp.ContainsKey(input[i]))
            {
                SeqComp.Add(input[i], 0);
            }
            SeqComp[input[i]]++;
        }

        foreach (var item in SeqComp)
        {
            Console.Write($"{item.Value}{item.Key} ");
        }

        sw.Stop(); 

        Console.WriteLine($"\n Time: {sw.ElapsedMilliseconds} ms");
    }
}

