import { Word } from '../index';

test('Word', () => {
	const word = new Word('hello');
	expect(word.toString()).toBe('hello');
	expect(word.matches('hello world')).toEqual({ start: 0, end: 5 });
	expect(word.matches('world')).toBeNull();
});
