/**
 * Advent of Code 2023
 * Day 04
 *
 * URL: [Link to the challenge for Day {{DAY_NUMBER}}]
 */
// Import necessary libraries or modules
import readFile from "./utils.js";
import fs from "fs";
// Function to solve part 1 of the challenge

fs.readFile("input.txt", "utf-8", (err, data) => {
  const input = data.split("\n").map((x) => x.replace(/  /g, " 0"));

  const res = input.reduce((acc, row) => {
    const [, cards] = row.split(": ");
    const [winners, myCards] = cards.split(" | ");

    const point = myCards
      .split(" ")
      .filter((card) => winners.includes(card)).length;

    const value = point === 0 ? 0 : Math.pow(2, point - 1);
    console.log({ point, value });
    return acc + value;
  }, 0);

  console.log(res);
});
