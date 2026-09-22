export const TITLE_MAIN = "Slip";
export const TITLE_SEARCH = "Slip Search";
export const TITLE_TILED = "Slip Tiled";

export function editorTitle(isMain: boolean, noteTitle: string | undefined): string {
  return isMain || !noteTitle ? TITLE_MAIN : `${TITLE_MAIN} - ${noteTitle}`;
}
