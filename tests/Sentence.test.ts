import { Boundary, Sentence, Word } from '../index';

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
	});

	describe('toCensoredString', () => {
		test('GIVEN a matching word THEN returns censored string', () => {
			const original = 'Hello world';
			const sentence = new Sentence(original);
			const word = new Word('hello');

			expect(word.matches(sentence)).toBe(true);
			expect(sentence.toCensoredString({ original })).toBe('***** world');
		});

		test('GIVEN a matching word and a custom censor string THEN returns custom censored string', () => {
			const original = 'Hello world';
			const sentence = new Sentence(original);
			const word = new Word('hello');

			expect(word.matches(sentence)).toBe(true);
			expect(sentence.toCensoredString({ original, character: '\\*' })).toBe('\\*\\*\\*\\*\\* world');
		});

		test('GIVEN a non-matching word THEN returns censored string', () => {
			const original = 'Hello world';
			const sentence = new Sentence(original);
			const word = new Word('foo');

			expect(word.matches(sentence)).toBe(false);
			expect(sentence.toCensoredString({ original })).toBe('Hello world');
		});
	});
});
