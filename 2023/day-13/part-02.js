/**
 * Advent of Code 2023
 * Day 13
 *
 * URL: [Link to the challenge for Day {{DAY_NUMBER}}]
 */
// Import necessary libraries or modules
import fs from "fs";
// Function to solve part 1 of the challenge
const getDifferencesCount = (row1, row2) => {
  let differences = 0;
  let i = 0;
  while (i < row1.length) {
    if (row1[i] !== row2[i]) differences++;
    i++;
  }
  return differences;
};

const vReflect = (section) => {
  let len = section.length;
  for (let row = 0; row < len - 1; row++) {
    let anomaly = 0;
    let t = row;
    let b = row + 1;

    while (t >= 0 && b < len) {
      let topRow = section[t];
      let bottomRow = section[b];
      anomaly += getDifferencesCount(topRow, bottomRow);
      t--;
      b++;
    }
    if (anomaly === 1) return (row + 1) * 100;
  }
  return null;
};

const hReflect = (section) => {
  let len = section[0].length;
  for (let col = 0; col < len - 1; col++) {
    let anomaly = 0;
    let l = col;
    let r = col + 1;
    while (l >= 0 && r < len) {
      let leftCol = section.map((row) => row[l]).join("");
      let rightCol = section.map((row) => row[r]).join("");
      anomaly += getDifferencesCount(leftCol, rightCol);
      l--;
      r++;
    }
    if (anomaly === 1) return col + 1;
  }
  return null;
};

export const solvePart2 = () => {
  fs.readFile("input.txt", "utf-8", (err, data) => {
    const input = data.split("\n");
    let val = input
      .reduce(
        (acc, line) => {
          if (!line) {
            acc = [...acc, []];
          } else {
            acc.at(-1).push(line);
          }
          return acc;
        },
        [[]],
      )
      .reduce((acc, part) => {
        let h = hReflect(part);
        let v = vReflect(part);
        acc += h || v || 0;
        return acc;
      }, 0);
    console.log(val);
  });
};

solvePart2();
