/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export class Word {
	constructor(word: string);
	matches(sentence: string): WordMatch | null;
	toString(): string;
}
export class WordMatch {
	get start(): number;
	get end(): number;
}
