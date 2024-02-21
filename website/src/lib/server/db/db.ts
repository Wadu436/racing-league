import * as schema from './schema';
import { drizzle } from 'drizzle-orm/better-sqlite3';
import { migrate } from 'drizzle-orm/better-sqlite3/migrator';
import Database from 'better-sqlite3';

// const betterSqlite = new Database(':memory:');
export const sqliteDb = new Database('./drizzle.db');
export const db = drizzle(sqliteDb, { schema });

console.log('Migrating database...');
try {
	migrate(db, { migrationsFolder: 'drizzle' });
} catch (e) {
	console.error('Error while migrating database:', e);
}
