import { CountrySchema } from '$lib/flags/countryCodes';
import { object, string, boolean, optional, length, minLength } from 'valibot';

export const formSchema = object({
	username: string([minLength(1)]),
	admin: boolean(),
	staff: boolean(),
});

export type FormSchema = typeof formSchema;