// https://orm.drizzle.team/docs/sql-schema-declaration
import { sqliteTable, text, integer, primaryKey } from 'drizzle-orm/sqlite-core';

// Lucia Auth stuff
export const user = sqliteTable('user', {
	id: text('id').primaryKey(),
	// other user attributes
	username: text('username').unique().notNull()
});

export const session = sqliteTable('session', {
	id: text('id').primaryKey(),
	userId: text('user_id')
		.notNull()
		.references(() => user.id),
	expires_at: integer('expires_at').notNull()
});

export const oauth_account = sqliteTable(
	'oauth_account',
	{
		providerId: text('provider_id').notNull(),
		providerUserId: text('provider_user_id').notNull(),
		userId: text('user_id')
			.notNull()
			.references(() => user.id)
	},
	(table) => {
		return {
			pk: primaryKey({ columns: [table.providerId, table.providerUserId] })
		};
	}
);

export const newUsers = sqliteTable('new_users', {
	id: text('id').primaryKey(),
	providerId: text('provider_id').notNull(),
	providerUserId: text('provider_user_id').notNull(),
	expirationTime: integer('expiration_time', { mode: 'timestamp' }).notNull()
});
