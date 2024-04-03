import { object, string, minLength } from 'valibot';

export const newLeagueSchema = object({
	name: string([minLength(1)])
});

export type FormSchema = typeof newLeagueSchema;
