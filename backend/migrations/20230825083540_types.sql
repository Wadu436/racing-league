-- Add migration script here
CREATE TYPE LeagueStatus AS ENUM ('Active', 'Finished', 'Planned');
CREATE TYPE FinishStatus AS ENUM ('Classified', 'Dnf', 'Dns', 'Dsq', 'Dnq');
CREATE TYPE SessionType AS ENUM (
    'Race',
    'Q1',
    'Q2',
    'Q3',
    'ShortQualifying',
    'OneShotQualifying',
    'Sprint',
    'SprintQualifying',
    'Practice'
);
CREATE TYPE LapType AS ENUM ('In', 'Out', 'Hot', 'Sc', 'Vsc');
CREATE TYPE TyreType AS ENUM ('Soft', 'Medium', 'Hard', 'Inter', 'Wet');
CREATE TYPE UserRole AS ENUM ('User', 'Manager', 'Admin');