import { Part01 } from "./part01";

test("returns total", async () => {
  const aws = await Part01();
  expect(aws).toBe(55447);
});
