/**
 * Advent of Code 2023
 * Day 03
 *
 * URL: [Link to the challenge for Day {{DAY_NUMBER}}]
 */
// Import necessary libraries or modules
import readFile from "./utils.js";
// Function to solve part 1 of the challenge

const hasSymbol = (str) => {
  if (str?.length && str.split("").find((x) => isNaN(x) && x !== "."))
    return true;
  return false;
};

export const solvePart02 = async () => {
  const data = await readFile("input.txt");
  const rows = data.length;
  const cols = data[0].length;
  const gearsDic = {};

  const findGears = (str, num, i, j) => {
    j = j === -1 ? 0 : j;
    for (let k = 0; k < str.length; k++) {
      const ch = str.charAt(k);
      if (ch === "*") {
        const ind = `${i}-${j + k}`;
        gearsDic[ind] = gearsDic[ind]
          ? [...gearsDic[ind], parseInt(num)]
          : [parseInt(num)];
      }
    }
  };

  for (let i = 0; i < rows; i++) {
    for (let j = 0; j < cols; j++) {
      const element = "" + data[i][j];
      if (isNaN(element)) continue;

      let num = element;
      while (++j < cols) {
        if (Number.isInteger(parseInt(data[i][j]))) {
          num += data[i][j];
        } else break;
      }
      const top =
        i === 0 ? "" : data[i - 1].substring(j - num.length - 1, j + 1);
      const btm =
        i === rows - 1 ? "" : data[i + 1].substring(j - num.length - 1, j + 1);
      const lft = data[i][j - num.length - 1] || "";
      const rgt = data[i][j] || "";

      findGears(top, num, i - 1, j - num.length - 1);
      findGears(btm, num, i + 1, j - num.length - 1);
      findGears(lft, num, i, j - num.length - 1);
      findGears(rgt, num, i, j);
    }
  }
  const gearKeys = Object.keys(gearsDic);
  let gearRatioProducts = [];
  gearKeys.forEach((key) => {
    if (gearsDic[key].length === 2) {
      gearRatioProducts.push(gearsDic[key][0] * gearsDic[key][1]);
    }
  });
  console.log(gearsDic);
  console.log(gearKeys);
  console.log(gearRatioProducts.reduce((acc, cv) => acc + cv, 0));
};
solvePart02();
