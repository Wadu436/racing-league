import { CountrySchema } from '$lib/flags/countryCodes';
import { object, string, boolean, optional, length, minLength } from 'valibot';

export const formSchema = object({
	name: string([minLength(1)]),
	country: CountrySchema,
	bot: boolean()
});

export type FormSchema = typeof formSchema;
