import { Boundary, Sentence } from '../index';

describe('Sentence', () => {
	test.each([
		'hello world', //
		'𝕙ȩ𝕀𝓁ṓ ẁọʳ𝓘ď',
		'𝔥⒠𝚕Ӏő ὦ𝟶ɼıᑱ'
	])("GIVEN a basic string THEN it's read correctly", (input) => {
		const sentence = new Sentence(input);
		expect(sentence.toString()).toBe('hello world');
		expect(sentence.length).toBe(11);
		expect(sentence.boundaries).toEqual([
			Boundary.Start, //     H
			Boundary.Word, //      e
			Boundary.Word, //      l
			Boundary.Word, //      l
			Boundary.End, //       o
			Boundary.NoContent, // (space)
			Boundary.Start, //     w
			Boundary.Word, //      o
			Boundary.Word, //      r
			Boundary.Word, //      l
			Boundary.End //        d
		]);
		expect(sentence.checked).toEqual([false, false, false, false, false, false, false, false, false, false, false]);
	});
});
