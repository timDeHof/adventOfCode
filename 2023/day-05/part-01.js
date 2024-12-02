/**
 * Advent of Code 2023
 * Day 05
 *
 * URL: [Link to the challenge for Day {{DAY_NUMBER}}]
 */
// Import necessary libraries or modules
// import readFile from "./utils.js"
import fs from "fs";
// Function to solve part 1 of the challenge
const mapXtoY = (mapping, X) => {
  let Y = -1;
  for (const row of mapping) {
    const [end, start, count] = row.split(" ").map(Number);
    if (X >= start && X <= start + count) {
      Y = end - start + X;
      break;
    }
  }
  return Y === -1 ? X : Y;
};

export const solvePart1 = () => {
  fs.readFile("input.txt", "utf-8", (err, data) => {
    const input = data.split("\n\n");

    const inputSeeds = input[0]
      .split("seeds: ")
      .filter((x) => x)[0]
      .split(" ")
      .map((seed) => parseInt(seed.trim()));

    let [, ...seed_to_soil_map] = input[1].split("\n");
    let [, ...soil_to_Fertilizer_map] = input[2].split("\n");
    let [, ...fertilizer_to_water_map] = input[3].split("\n");
    let [, ...water_to_light_map] = input[4].split("\n");
    let [, ...light_to_temperature_map] = input[5].split("\n");
    let [, ...temperature_to_humidity_map] = input[6].split("\n");
    let [, ...humidity_to_location_map] = input[7].split("\n");

    let res = inputSeeds
      .map((n) => mapXtoY(seed_to_soil_map, n))
      .map((n) => mapXtoY(soil_to_Fertilizer_map, n))
      .map((n) => mapXtoY(fertilizer_to_water_map, n))
      .map((n) => mapXtoY(water_to_light_map, n))
      .map((n) => mapXtoY(light_to_temperature_map, n))
      .map((n) => mapXtoY(temperature_to_humidity_map, n))
      .map((n) => mapXtoY(humidity_to_location_map, n));

    console.log(Math.min(...res));
  });
};
//# sourceMappingURL=dayTemplate.js.map
solvePart1();
