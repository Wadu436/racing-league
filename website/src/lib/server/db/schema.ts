// https://orm.drizzle.team/docs/sql-schema-declaration
import { relations } from 'drizzle-orm';
import { sqliteTable, text, integer, primaryKey, int, foreignKey } from 'drizzle-orm/sqlite-core';

// Lucia Auth stuff
export const users = sqliteTable('users', {
	id: text('id').primaryKey(),
	// other user attributes
	username: text('username').unique().notNull(),
	admin: integer('admin', { mode: 'boolean' }).notNull(),
	staff: integer('staff', { mode: 'boolean' }).notNull()
});

export const sessions = sqliteTable('sessions', {
	id: text('id').primaryKey(),
	userId: text('user_id')
		.notNull()
		.references(() => users.id),
	expires_at: integer('expires_at').notNull()
});

export const oauthAccounts = sqliteTable(
	'oauth_accounts',
	{
		providerId: text('provider_id').notNull(),
		providerUserId: text('provider_user_id').notNull(),
		userId: text('user_id')
			.notNull()
			.references(() => users.id)
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

// Data stuff
export const drivers = sqliteTable('drivers', {
	id: text('id').primaryKey(),
	name: text('name').notNull().unique(),
	bot: integer('bot', { mode: 'boolean' }).notNull(),
	country: text('country', { length: 2 }).notNull()
});

export const tracks = sqliteTable('tracks', {
	id: text('id').primaryKey(),
	name: text('name').notNull(),
	country: text('country', { length: 2 }).notNull(),
	image_path: text('image_path')
});

export const teams = sqliteTable('teams', {
	id: text('id').primaryKey(),
	name: text('name').notNull(),
	country: text('country', { length: 2 }).notNull(),
	image_path: text('image_path')
});

export const games = sqliteTable('games', {
	id: text('id').primaryKey(),
	name: text('name').notNull(),
	image_path: text('image_path')
});

export const leagues = sqliteTable('leagues', {
	id: text('id').primaryKey(),
	name: text('name').notNull(),
	status: text('status', { enum: ['upcoming', 'ongoing', 'finished'] }).notNull(),
	gameId: text('game_id').references(() => games.id)
});

export const leaguesRelations = relations(leagues, ({ one }) => ({
	game: one(games, { fields: [leagues.gameId], references: [games.id] })
}));

export const events = sqliteTable('events', {
	id: text('id').primaryKey(),
	name: text('name').notNull(),
	leagueOrder: int('league_order').notNull(),
	date: int('date', { mode: 'timestamp' }),
	leagueId: text('league_id')
		.notNull()
		.references(() => leagues.id, { onDelete: 'cascade' }),
	trackId: text('track_id')
		.notNull()
		.references(() => tracks.id)
});

export const eventsRelations = relations(events, ({ one }) => ({
	track: one(tracks, { fields: [events.trackId], references: [tracks.id] })
}));

export const tracksRelations = relations(tracks, ({ one }) => ({
	events: one(events, { fields: [tracks.id], references: [events.trackId] })
}));

export const eventSessions = sqliteTable('event_sessions', {
	id: text('id').primaryKey(),
	eventId: text('event_id')
		.notNull()
		.references(() => events.id, { onDelete: 'cascade' }),
	session_type: text('session_type', {
		enum: [
			'Race',
			'Q1',
			'Q2',
			'Q3',
			'ShortQualifying',
			'OneShotQualifying',
			'Sprint',
			'SprintQualifying',
			'Practice'
		]
	}).notNull(),
	fastest_lap: integer('fastest_lap').references(() => drivers.id)
});

export const leagueEntries = sqliteTable(
	'league_entries',
	{
		leagueId: text('league_id')
			.notNull()
			.references(() => leagues.id, { onDelete: 'cascade' }),
		driverId: text('driver_id')
			.notNull()
			.references(() => drivers.id),
		teamId: text('team_id')
			.notNull()
			.references(() => teams.id)
	},
	(table) => {
		return { pk: primaryKey({ columns: [table.leagueId, table.driverId] }) };
	}
);

export const sessionEntries = sqliteTable(
	'session_entries',
	{
		sessionId: text('session_id')
			.notNull()
			.references(() => sessions.id, { onDelete: 'cascade' }),
		driverId: text('driver_id')
			.notNull()
			.references(() => drivers.id),
		teamId: text('team_id')
			.notNull()
			.references(() => teams.id),
		finish_status: text('finish_status', {
			enum: ['Finished', 'DNF', 'DNS', 'DSQ', 'DNQ']
		}).notNull(),
		grid_position: int('grid_position').notNull(),
		finish_position: int('finish_position').notNull(),
		total_time_without_penalties_in_ms: int('total_time_without_penalties_in_ms').notNull(),
		penalty_time_in_seconds: int('penalty_time_in_s').notNull()
	},
	(table) => {
		return {
			pk: primaryKey({ columns: [table.sessionId, table.driverId] })
		};
	}
);

export const laps = sqliteTable(
	'laps',
	{
		sessionId: text('session_id')
			.notNull()
			.references(() => sessions.id),
		driverId: text('driver_id')
			.notNull()
			.references(() => drivers.id),
		lap_number: int('lap_number').notNull(),
		laptime_in_ms: int('laptime_in_ms').notNull(),
		sector1TimeInMs: int('sector_1_time_in_ms').notNull(),
		sector2TimeInMs: int('sector_2_time_in_ms').notNull(),
		sector3TimeInMs: int('sector_3_time_in_ms').notNull(),
		valid: integer('valid', { mode: 'boolean' }).notNull(),
		inLap: integer('in_lap', { mode: 'boolean' }).notNull(),
		outLap: integer('out_lap', { mode: 'boolean' }).notNull(),
		safetyCar: integer('safety_car', { mode: 'boolean' }).notNull(),
		virtualSafetyCar: integer('virtual_safety_car', { mode: 'boolean' }).notNull()
	},
	(table) => {
		return {
			pk: primaryKey({ columns: [table.sessionId, table.driverId, table.lap_number] }),
			sessionEntryReference: foreignKey(() => ({
				name: 'session_entry_reference',
				columns: [table.sessionId, table.driverId],
				foreignColumns: [sessionEntries.sessionId, sessionEntries.driverId]
			})).onDelete('cascade')
		};
	}
);

export const tyreStints = sqliteTable(
	'tyre_stints',
	{
		sessionId: text('session_id')
			.notNull()
			.references(() => sessions.id),
		driverId: text('driver_id')
			.notNull()
			.references(() => drivers.id),
		order: int('order').notNull(),
		compound: text('compound', { enum: ['soft', 'medium', 'hard', 'inter', 'wet'] }).notNull(),
		startLap: int('start_lap').notNull(),
		endLap: int('end_lap').notNull()
	},
	(table) => {
		return {
			pk: primaryKey({ columns: [table.sessionId, table.driverId, table.order] }),
			session_entry_reference: foreignKey(() => ({
				name: 'session_entry_reference',
				columns: [table.sessionId, table.driverId],
				foreignColumns: [sessionEntries.sessionId, sessionEntries.driverId]
			})).onDelete('cascade')
		};
	}
);

export const overtakes = sqliteTable(
	'overtakes',
	{
		id: text('id').primaryKey(),
		sessionId: text('session_id')
			.notNull()
			.references(() => sessions.id, { onDelete: 'cascade' }),
		overtakingDriverId: text('overtaking_driver_id')
			.notNull()
			.references(() => drivers.id),
		overtakenDriverId: text('overtaken_driver_id')
			.notNull()
			.references(() => drivers.id),
		lapNumber: int('lap_number').notNull()
	},
	(table) => ({
		overtakingSessionEntryReference: foreignKey(() => ({
			name: 'overtaking_session_entry_reference',
			columns: [table.sessionId, table.overtakingDriverId],
			foreignColumns: [sessionEntries.sessionId, sessionEntries.driverId]
		})),
		overtakenSessionEntryReference: foreignKey(() => ({
			name: 'overtaken_session_entry_reference',
			columns: [table.sessionId, table.overtakenDriverId],
			foreignColumns: [sessionEntries.sessionId, sessionEntries.driverId]
		}))
	})
);
