-- Add migration script here
CREATE TABLE users (
    id uuid PRIMARY KEY,
    username TEXT,
    profile_picture_path TEXT,
    steam_id TEXT,
    ea_id TEXT,
    UNIQUE (steam_id),
    UNIQUE (ea_id)
);
CREATE TABLE roles (
    user_id uuid NOT NULL,
    role UserRole NOT NULL,
);
CREATE TABLE drivers (
    id uuid PRIMARY KEY,
    bot BOOLEAN NOT NULL,
    name TEXT NOT NULL,
    nationality CHAR(2),
    steam_id TEXT,
    ea_id TEXT,
    UNIQUE (steam_id),
    UNIQUE (ea_id)
);
CREATE TABLE tracks (
    id uuid PRIMARY KEY,
    name TEXT NOT NULL,
    country CHAR(2) NOT NULL,
    image_path TEXT
);
CREATE TABLE teams (
    id uuid PRIMARY KEY,
    name TEXT NOT NULL,
    nationality CHAR(2) NOT NULL,
    image_path TEXT
);
CREATE TABLE leagues (
    id uuid PRIMARY KEY,
    name TEXT NOT NULL,
    status LeagueStatus NOT NULL
);
CREATE TABLE events (
    id uuid PRIMARY KEY,
    name TEXT NOT NULL,
    championship_order INT NOT NULL,
    date TIMESTAMPTZ NOT NULL,
    league_id uuid NOT NULL,
    FOREIGN KEY (league_id) REFERENCES leagues(id) ON DELETE CASCADE,
    track_id uuid NOT NULL,
    FOREIGN KEY (track_id) REFERENCES tracks(id)
);
CREATE TABLE sessions (
    id uuid PRIMARY KEY,
    event_id uuid NOT NULL,
    session_type SessionType NOT NULL,
    fastest_lap uuid NOT NULL,
    FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE CASCADE,
    FOREIGN KEY (fastest_lap) REFERENCES session_entries(id)
);
CREATE TABLE league_entries (
    league_id uuid NOT NULL,
    driver_id uuid NOT NULL,
    team_id uuid NOT NULL,
    PRIMARY KEY (league_id, driver_id),
    FOREIGN KEY (league_id) REFERENCES leagues(id) ON DELETE CASCADE,
    FOREIGN KEY (driver_id) REFERENCES drivers(id),
    FOREIGN KEY (team_id) REFERENCES teams(id)
);
CREATE TABLE session_entries (
    session_id uuid NOT NULL,
    driver_id uuid NOT NULL,
    team_id uuid NOT NULL,
    finish_status FinishStatus NOT NULL,
    grid_position int,
    finish_position int NOT NULL,
    PRIMARY KEY (session_id, driver_id),
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE,
    FOREIGN KEY (driver_id) REFERENCES drivers(id) FOREIGN KEY (team_id) REFERENCES teams(id)
);
CREATE TABLE laps (
    id uuid PRIMARY KEY,
    session_id uuid NOT NULL,
    driver_id uuid NOT NULL,
    lap_number int NOT NULL,
    laptime_in_ms int NOT NULL,
    valid BOOLEAN NOT NULL,
    lap_type LapType NOT NULL,
    tyres TyreType NOT NULL,
    UNIQUE (session_id, driver_id, lap_number),
    FOREIGN KEY (session_id, driver_id) REFERENCES session_entries(session_id, driver_id) ON DELETE CASCADE
);
CREATE TABLE overtake (
    id uuid PRIMARY KEY,
    session_id uuid NOT NULL,
    overtaking_driver_id uuid NOT NULL,
    overtaken_driver_id uuid NOT NULL,
    lap_number int NOT NULL,
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE,
    FOREIGN KEY (session_id, overtaking_driver_id) REFERENCES session_entries(session_id, driver_id),
    FOREIGN KEY (session_id, overtaken_driver_id) REFERENCES session_entries(session_id, driver_id)
);