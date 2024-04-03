import { object, string, minLength, optional } from 'valibot';

export const newLeagueSchema = object({
	name: string([minLength(1)]),
	gameId: optional(string())
});

export type FormSchema = typeof newLeagueSchema;
