import { Variables } from "@fermyon/spin-sdk";

const loadConfig = (metadata, request) => {
    metadata.config = {
        dbConnectionString: Variables.get("db_connection_string")
    };
};

export {
    loadConfig
};