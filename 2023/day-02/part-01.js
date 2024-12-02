/**
 * Advent of Code 2023
 * Day 02
 *
 * URL: https://adventofcode.com/2023/day/2
 */
// Import necessary libraries or modules
import readFile from "./utils.js";
// Function to solve part 1 of the challenge
const getGameNumber = (game) => Number(game.split(":")[0].split(" ")[1]);
const getBagColors = (input) => {
  const [, bagsString] = input.split(": ");
  const segments = bagsString.split("; ");

  const result = segments.map((segment) => {
    const colors = segment.split(", ");
    // console.log({ colors });
    const bag = { red: 0, green: 0, blue: 0 }; // Initialize counts to 0 instead of null
    colors.forEach((colorCount) => {
      let [count, color] = colorCount.split(" ");
      count = parseInt(count, 10);
      color = color.toLowerCase();
      if (bag.hasOwnProperty(color)) {
        bag[color] = count; // Use += to add the count to the existing value
      }
    });
    console.log({ bag });
    return bag;
  });
  return result;
};

const checkBagCubes = (bag) => {
  let result = true;
  if (bag.red > idealBag.red) {
    result = false;
  }
  if (bag.green > idealBag.green) {
    result = false;
  }
  if (bag.blue > idealBag.blue) {
    result = false;
  }

  return result;
};

function getValidGameNumbers(games) {
  let validGamesArr = [];
  games.forEach(({ number, bags }) => {
    let validBag = true;
    bags.forEach((bag) => {
      if (!checkBagCubes(bag)) {
        validBag = false;
      }
    });
    if (validBag) {
      validGamesArr.push(number);
    }
  });
  return validGamesArr;
}

function calculateIdSum(validIds) {
  return validIds.reduce((acc, cv) => acc + cv, 0);
}

const idealBag = {
  red: 12,
  green: 13,
  blue: 14,
};
export const solvePart1 = async () => {
  try {
    const data = await readFile("input.txt");
    // Your solution for part 1 goes here

    const games = data.map((game) => {
      return {
        number: getGameNumber(game),
        bags: getBagColors(game),
      };
    });
    const validGames = getValidGameNumbers(games);
    console.log(validGames);
    const idSum = calculateIdSum(validGames);
    console.log(idSum);
  } catch (error) {
    console.error("An error occurred:", error);
  }
};
solvePart1();
