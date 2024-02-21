export const formDataToRecord = (formData: FormData): Record<string, FormDataEntryValue> => {
	const record: Record<string, FormDataEntryValue> = {};
	for (const pair of formData.entries()) {
		record[pair[0]] = pair[1];
	}
	return record;
};
