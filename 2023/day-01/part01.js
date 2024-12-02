import readFile from "./parseFile.js";

const findFirstNumber = (line) => {
  for (let i = 0; i < line.length; i++) {
    if (!isNaN(parseInt(line[i], 10))) {
      // Check if character at index i is a digit
      return line[i]; // Assign the digit to firstDigit
      break; // Exit the loop since we've found the first digit
    }
  }
};

const findLastNumber = (line) => {
  for (let i = line.length; i >= 0; i--) {
    if (!isNaN(parseInt(line[i], 10))) {
      // Check if character at index i is a digit
      return line[i]; // Assign the digit to firstDigit
      break; // Exit the loop since we've found the first digit
    }
  }
};

export const Part01 = async () => {
  const data = await readFile("input.txt");

  let total = 0;
  data.forEach((line) => {
    let firstDigit = findFirstNumber(line);
    let secondDigit = findLastNumber(line);
    total += Number(firstDigit + secondDigit);
  });
  return total;
};

console.log(await Part01());
