import { Sqlite } from "@fermyon/spin-sdk"

const DB_NAME = "crud";

const seed = () => {
    const db = Sqlite.open(DB_NAME);
    db.execute("CREATE TABLE IF NOT EXISTS VehicleFeatures (Id VARCHAR(36) PRIMARY KEY, Name TEXT NOT NULL, Active BOOLEAN)")
}
export {
    seed
}