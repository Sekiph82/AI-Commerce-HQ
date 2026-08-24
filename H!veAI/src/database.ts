export type DatabaseStatus = {
  initialized: boolean;
  engine: string;
  schemaVersion: number;
  migrationCount: number;
  databasePath: string;
  foreignKeysEnabled: boolean;
  lastMigrationStatus: string;
};
