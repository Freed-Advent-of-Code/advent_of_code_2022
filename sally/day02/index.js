const score = {
  win: 6,
  draw: 3,
  lose: 0,
};
const inputs = require("fs")
  .readFileSync("sally/day02/inputs.txt")
  .toString()
  .split("\n");
const subAscii = (char1, char2) => char1.charCodeAt(0) - char2.charCodeAt(0);
const getScore1 = (opp, me) => {
  if (opp === me) return me + 1 + score.draw;
  if ((opp + 1) % 3 === me) return me + 1 + score.win;
  return me + 1 + score.lose;
};
const getScore2 = (opp, result) => {
  if (result === score.win) return score.win + ((opp + 1) % 3) + 1;
  if (result === score.draw) return score.draw + opp + 1;
  return ((opp + 2) % 3) + 1;
};
const getTotalScore1 = () =>
  inputs
    .map((game) => getScore1(subAscii(game[0], "A"), subAscii(game[2], "X")))
    .reduce((sum, score) => sum + score);
const getTotalScore2 = () =>
  inputs
    .map((game) =>
      getScore2(subAscii(game[0], "A"), subAscii(game[2], "X") * 3)
    )
    .reduce((sum, score) => sum + score);
console.log(getTotalScore1());
console.log(getTotalScore2());
