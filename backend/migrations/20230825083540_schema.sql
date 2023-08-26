-- Add migration script here
CREATE TYPE LeagueStatus AS ENUM ("Active", "Finished");
CREATE TYPE FinishStatus AS ENUM ("Finished", "DNF", "DNS", "DSQ");
CREATE TYPE SessionType AS ENUM (
    "Practice",
    "Qualifying",
    "SprintQualifying",
    "Race",
    "Sprint"
);
CREATE TABLE IF NOT EXISTS users (
    id uuid PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    nationality CHAR(2),
    image_path TEXT,
);
CREATE TABLE IF NOT EXISTS tracks (
    id uuid PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    country CHAR(2) NOT NULL,
    image_path TEXT,
);
CREATE TABLE IF NOT EXISTS teams (
    id uuid PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    nationality CHAR(2) NOT NULL,
    image_path TEXT,
);
CREATE TABLE IF NOT EXISTS leagues (
    id uuid PRIMARY KEY,
    name TEXT NOT NULL,
    status LeagueStatus NOT NULL,
);
CREATE TABLE IF NOT EXISTS events (
    id uuid PRIMARY KEY,
    name TEXT NOT NULL,
    championship_order INT NOT NULL,
    date TIMESTAMPTZ NOT NULL,
    league_id uuid NOT NULL,
    FOREIGN KEY (league_id) REFERENCES leagues(id) ON DELETE CASCADE,
    track_id uuid NOT NULL,
    FOREIGN KEY (track_id) REFERENCES tracks(id),
);
CREATE TABLE IF NOT EXISTS sessions (
    id uuid PRIMARY KEY,
    event_id uuid NOT NULL,
    session_type SessionType NOT NULL,
    fastest_lap uuid NOT NULL,
    
    FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE CASCADE,
    FOREIGN KEY (fastest_lap) REFERENCES users(id),
);