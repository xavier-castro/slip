import assert from "node:assert/strict";
import test from "node:test";
import { TITLE_SEARCH, TITLE_TILED, editorTitle } from "../src/titles.ts";

test("editor titles match the Slip window rules", () => {
  assert.equal(editorTitle(true, "Grocery"), "Slip");
  assert.equal(editorTitle(false, undefined), "Slip");
  assert.equal(editorTitle(false, "Grocery"), "Slip - Grocery");
  assert.equal(TITLE_SEARCH, "Slip Search");
  assert.equal(TITLE_TILED, "Slip Tiled");
});
