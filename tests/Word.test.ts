import { Word, type WordMatch } from '../index';

test('Word', () => {
	const word = new Word('hello');
	expect(word.toString()).toBe('hello');
	expect(toObject(word.matches('hello world'))).toEqual({ start: 0, end: 4 });
	expect(word.matches('world')).toBeNull();
});

function toObject(value: WordMatch | null) {
	return value ? ({ start: value.start, end: value.end } as const satisfies WordMatch) : null;
}
