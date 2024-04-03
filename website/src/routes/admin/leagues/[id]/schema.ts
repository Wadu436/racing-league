import { object, string, minLength, picklist } from 'valibot';

export const leagueInfoSchema = object({
	name: string([minLength(1)]),
	status: picklist(['upcoming', 'ongoing', 'finished'])
});

export type FormSchema = typeof leagueInfoSchema;
