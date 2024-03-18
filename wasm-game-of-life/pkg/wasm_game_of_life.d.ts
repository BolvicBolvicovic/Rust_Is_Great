/* tslint:disable */
/* eslint-disable */
/**
*/
export class Universe {
  free(): void;
/**
*/
  tick(): void;
/**
* @returns {Universe}
*/
  static new(): Universe;
/**
* @returns {number}
*/
  get_width(): number;
/**
* @returns {number}
*/
  get_height(): number;
/**
* @returns {number}
*/
  get_cells(): number;
/**
* @returns {string}
*/
  render(): string;
/**
*/
  clear_cells(): void;
/**
* @param {number} row
* @param {number} column
*/
  toggle_cell(row: number, column: number): void;
/**
*/
  random_restart(): void;
/**
* @param {number} row
* @param {number} column
*/
  glider(row: number, column: number): void;
/**
* @param {number} row
* @param {number} column
*/
  pulsar(row: number, column: number): void;
/**
* @param {number} row
* @param {number} column
*/
  pentadecathlon(row: number, column: number): void;
}
