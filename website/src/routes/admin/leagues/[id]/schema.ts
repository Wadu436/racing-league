import { object, string, minLength, picklist, optional, nullable } from 'valibot';

export const leagueInfoSchema = object({
	name: string([minLength(1)]),
	status: picklist(['upcoming', 'ongoing', 'finished']),
	gameId: nullable(optional(string()))
});

export type FormSchema = typeof leagueInfoSchema;
