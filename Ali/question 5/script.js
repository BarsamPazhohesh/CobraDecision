/*This could be the easiest interview question you might be asked if you were applying for Microsoft.

Imagine if you have a sequence of repeated characters like:

AAAABBB1111CCCDD

Your goal is to find a way to compress this sequence and make it shorter while still being able to expand it back to the original.
Example:

AAAABBB1111CCCDD => 4A 3B 41 3C 2D*/

// 1:
const calculateAlphabets = function (word) {
  return [...word].reduce((acc, char) => {
    acc[char] = (acc[char] || 0) + 1;
    return acc;
  }, {});
};

console.log(calculateAlphabets("AAAABBB1111CCCDD"));

//2:
const char = "AAAABBB1111CCCDD";
const calculateAlphabets2 = function (word2) {
  const count = {};

  for (let i = 0; i < word2.length; i++) {
    const letter = word2[i];

    count[letter] = (count[letter] || 0) + 1;
  }

  console.log(count);
};
calculateAlphabets2([...char]);
