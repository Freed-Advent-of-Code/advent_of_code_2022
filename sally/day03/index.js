const inputs = require("fs")
  .readFileSync("sally/day03/input.txt")
  .toString()
  .split("\n");
const subAscii = (char1, char2) => char1.charCodeAt(0) - char2.charCodeAt(0);
const getPrioritySum = () => {
  return inputs
    .map((input) => {
      for (let i = 0; i < input.length / 2; i++) {
        for (let j = input.length / 2; j < input.length; j++) {
          if (input[i] === input[j]) {
            return input[i].charCodeAt(0) >= "a".charCodeAt(0)
              ? subAscii(input[i], "a") + 1
              : subAscii(input[i], "A") + 27;
          }
        }
      }
    })
    .reduce((sum, priority) => sum + priority);
};
console.log(getPrioritySum());
