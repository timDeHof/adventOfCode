/**
 * Advent of Code 2023
 * Day 02
 *
 * URL: https://adventofcode.com/2023/day/2
 */
// Import necessary libraries or modules
import readFile from "./utils.js";
// Function to solve part 1 of the challenge

export const solvePart2 = async () => {
  try {
    const data = await readFile("input2.txt");
    // Your solution for part 1 goes here
    let sumOfPowers = 0;
    data.forEach((line) => {
      const game = line.split(": ")[1];
      const values = game.replaceAll(";", ",").split(", ");
      let power = 1;
      ["blue", "green", "red"].forEach((color) => {
        const maxColorValue = Math.max(
          ...values
            .filter((value) => value.includes(color))
            .map((value) => Number(value.split(" ")[0])),
        );
        power *= maxColorValue;
      });
      sumOfPowers += power;
    });
    return sumOfPowers;
  } catch (error) {
    console.error("An error occurred:", error);
  }
};
console.log(await solvePart2());
