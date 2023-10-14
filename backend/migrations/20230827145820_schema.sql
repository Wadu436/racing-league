-- Add migration script here
CREATE TABLE IF NOT EXISTS  users (
    id uuid PRIMARY KEY,
    sub TEXT NOT NULL,
    username TEXT NOT NULL,
    profile_picture_path TEXT,
    steam_id TEXT,
    ea_id TEXT,
    UNIQUE (sub),
    UNIQUE (steam_id),
    UNIQUE (ea_id)
);
CREATE TABLE IF NOT EXISTS  roles (
    user_id uuid NOT NULL,
    role UserRole NOT NULL
);
CREATE TABLE IF NOT EXISTS  drivers (
    id uuid PRIMARY KEY,
    bot BOOLEAN NOT NULL,
    name TEXT NOT NULL,
    nationality CHAR(2),
    steam_id TEXT,
    ea_id TEXT,
    UNIQUE (steam_id),
    UNIQUE (ea_id)
);
CREATE TABLE IF NOT EXISTS  tracks (
    id uuid PRIMARY KEY,
    name TEXT NOT NULL,
    country CHAR(2) NOT NULL,
    image_path TEXT
);
CREATE TABLE IF NOT EXISTS  teams (
    id uuid PRIMARY KEY,
    name TEXT NOT NULL,
    nationality CHAR(2) NOT NULL,
    image_path TEXT
);
CREATE TABLE IF NOT EXISTS  leagues (
    id uuid PRIMARY KEY,
    name TEXT NOT NULL,
    status LeagueStatus NOT NULL
);
CREATE TABLE IF NOT EXISTS  events (
    id uuid PRIMARY KEY,
    name TEXT NOT NULL,
    championship_order INT NOT NULL,
    date TIMESTAMPTZ NOT NULL,
    league_id uuid NOT NULL,
    FOREIGN KEY (league_id) REFERENCES leagues(id) ON DELETE CASCADE,
    track_id uuid NOT NULL,
    FOREIGN KEY (track_id) REFERENCES tracks(id)
);
CREATE TABLE IF NOT EXISTS  sessions (
    id uuid PRIMARY KEY,
    event_id uuid NOT NULL,
    session_type SessionType NOT NULL,
    fastest_lap uuid NOT NULL,
    FOREIGN KEY (fastest_lap) REFERENCES drivers(id),
    FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS  league_entries (
    league_id uuid NOT NULL,
    driver_id uuid NOT NULL,
    team_id uuid NOT NULL,
    PRIMARY KEY (league_id, driver_id),
    FOREIGN KEY (league_id) REFERENCES leagues(id) ON DELETE CASCADE,
    FOREIGN KEY (driver_id) REFERENCES drivers(id),
    FOREIGN KEY (team_id) REFERENCES teams(id)
);
CREATE TABLE IF NOT EXISTS  session_entries (
    session_id uuid NOT NULL,
    driver_id uuid NOT NULL,
    team_id uuid NOT NULL,
    finish_status FinishStatus NOT NULL,
    grid_position int,
    finish_position int NOT NULL,
    PRIMARY KEY (session_id, driver_id),
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE,
    FOREIGN KEY (driver_id) REFERENCES drivers(id),
    FOREIGN KEY (team_id) REFERENCES teams(id)
);
ALTER TABLE sessions ADD CONSTRAINT sessions_id_fastest_lap_fkey FOREIGN KEY (id, fastest_lap) REFERENCES session_entries (session_id, driver_id);
CREATE TABLE IF NOT EXISTS  laps (
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
CREATE TABLE IF NOT EXISTS  overtake (
    id uuid PRIMARY KEY,
    session_id uuid NOT NULL,
    overtaking_driver_id uuid NOT NULL,
    overtaken_driver_id uuid NOT NULL,
    lap_number int NOT NULL,
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE,
    FOREIGN KEY (session_id, overtaking_driver_id) REFERENCES session_entries(session_id, driver_id),
    FOREIGN KEY (session_id, overtaken_driver_id) REFERENCES session_entries(session_id, driver_id)
);