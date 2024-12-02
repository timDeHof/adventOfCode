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

export const solvePart01 = async () => {
  const data = await readFile("input.txt");
  const rows = data.length;
  const cols = data[0].length;
  let partNumbers = [];

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

      if (
        hasSymbol(top) ||
        hasSymbol(btm) ||
        hasSymbol(lft) ||
        hasSymbol(rgt)
      ) {
        partNumbers.push(Number(num));
      }
    }
  }
  console.log(partNumbers);
  const sum = partNumbers.reduce((acc, cv) => acc + cv, 0);
  console.log(sum);
};
solvePart01();
