import { Config } from "@fermyon/spin-sdk";


const loadConfig = (request) => {
    request.config = {
        dbConnectionString: Config.get("db_connection_string") 
    };
};

export {
    loadConfig
};