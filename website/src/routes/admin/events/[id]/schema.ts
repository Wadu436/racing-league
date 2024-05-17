import { date, minLength, nullable, object, string } from 'valibot';

export const eventInfoSchema = object({
	name: string([minLength(1)]),
	date: nullable(date()),
	trackId: string()
});
