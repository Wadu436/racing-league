import { object, string, minLength } from 'valibot';

export const formSchema = object({
	name: string([minLength(1)])
});

export type FormSchema = typeof formSchema;
