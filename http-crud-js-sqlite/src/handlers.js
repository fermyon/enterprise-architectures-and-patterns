import { Sqlite } from "@fermyon/spin-sdk";
import { v4 as uuidv4 } from 'uuid';
import { validate as uuidValidate } from 'uuid';
const decoder = new TextDecoder();

const DB_NAME = "crud";
const COMMAND_CREATE_ITEM = "INSERT INTO VehicleFeatures (Id, Name, Active) VALUES ($1, $2, $3)";
const COMMAND_READ_ALL_ITEMS = "SELECT Id, Name, Active FROM VehicleFeatures ORDER BY Name";
const COMMAND_READ_SINGLE_ITEM = "SELECT Id, Name, Active FROM VehicleFeatures WHERE Id = $1";
const COMMAND_DELETE_SINGLE_ITEM = "DELETE FROM VehicleFeatures WHERE Id = $1";
const COMMAND_DELETE_MANY_ITEMS = "DELETE FROM VehicleFeatures WHERE Id IN";
const COMMAND_UPDATE_SINGLE_ITEM = "UPDATE VehicleFeatures SET Name = $1, Active = $2 WHERE Id = $3";

const DEFAULT_HEADERS = {
    "Content-Type": "application/json"
};

const getAllItems = () => {
    const db = Sqlite.open(DB_NAME);
    const queryResult = db.execute(COMMAND_READ_ALL_ITEMS, []);
    let items = queryResult.rows.map(row => {
        return {
            id: row[0],
            name: row[1],
            active: row[2]
        }
    });

    return {
        status: 200,
        headers: DEFAULT_HEADERS,
        body: JSON.stringify(items)
    };
};

const badRequest = (message) => {
    return {
        status: 400,
        headers: DEFAULT_HEADERS,
        body: JSON.stringify({
            message
        })
    };
};

const notFound = (message) => {
    return {
        status: 404,
        headers: DEFAULT_HEADERS,
        body: JSON.stringify({
            message
        })
    };
}

const getItemById = (id) => {
    if (!uuidValidate(id)) {
        return badRequest("Invalid identifier received via URL");
    }
    const db = Sqlite.open(DB_NAME);
    let queryResult = db.execute(COMMAND_READ_SINGLE_ITEM, [id]);
    if (queryResult.rows.length == 0) {
        return notFound(`No item found with id ${id}`);
    }
    let found = {
        id: queryResult.rows[0][0],
        name: queryResult.rows[0][1],
        active: queryResult.rows[0][2]
    }
    return {
        status: 200,
        headers: DEFAULT_HEADERS,
        body: found
    };
};

const deleteItemById = (id) => {
    if (!uuidValidate(id)) {
        return badRequest("Invalid identifier received via URL");
    }
    const db = Sqlite.open(DB_NAME);
    db.execute(COMMAND_DELETE_SINGLE_ITEM, [id]);
    return {
        status: 204,
        headers: DEFAULT_HEADERS,
        body: null
    };
};

const deleteManyItems = (requestBody) => {
    let payload = JSON.parse(decoder.decode(requestBody));
    if (!Array.isArray(payload) ||
        payload.length == 0 ||
        !payload.every(id => uuidValidate(id))) {
        return badRequest("Invalid payload received. Expecting an array with valid uuids");
    }

    let cmd = `${COMMAND_DELETE_MANY_ITEMS} (`;
    let parameters = [];
    for (let i = 0; i < payload.length; i++) {
        cmd = `${cmd}\$${i + 1}`;
        if (i < payload.length - 1) {
            cmd = `${cmd},`;
        }
        parameters.push(payload[i]);
    }
    cmd = `${cmd})`;
    const db = Sqlite.open(DB_NAME);
    db.execute(cmd, parameters);
    return {
        status: 204
    };
};

const createItem = (baseUrl, requestBody) => {
    let payload = JSON.parse(decoder.decode(requestBody));

    if (!payload || !payload.name || typeof payload.active != "boolean") {
        return badRequest("Invalid payload received. Expecting {\"name\":\"some name\", \"active\": true}");
    }

    const newItem = {
        id: uuidv4(),
        name: payload.name,
        active: payload.active
    };
    const db = Sqlite.open(DB_NAME);
    db.execute(COMMAND_CREATE_ITEM, [
        newItem.id,
        newItem.name,
        newItem.active
    ]);

    let customHeaders = {
        "Location": `${baseUrl}/items/${newItem.id}`
    };
    Object.assign(customHeaders, DEFAULT_HEADERS);

    return {
        status: 201,
        headers: customHeaders,
        body: JSON.stringify(newItem)
    };
};

const updateItemById = (baseUrl, id, requestBody) => {
    let payload = JSON.parse(decoder.decode(requestBody));
    if (!payload || !payload.name || typeof payload.active != "boolean") {
        return badRequest("Invalid payload received. Expecting {\"name\":\"some name\", \"active\": true}");
    }
    if (!uuidValidate(id)) {
        return badRequest("Invalid identifier received via URL");
    }
    const db = Sqlite.open(DB_NAME);
    db.execute(COMMAND_UPDATE_SINGLE_ITEM, [
        payload.name,
        payload.active,
        id
    ]);

    let customHeaders = {
        "Location": `${baseUrl}/items/${id}`
    };
    Object.assign(customHeaders, DEFAULT_HEADERS);

    return {
        status: 200,
        headers: customHeaders,
        body: null
    };
};

export {
    createItem,
    deleteItemById,
    deleteManyItems,
    getAllItems,
    getItemById,
    badRequest,
    notFound,
    updateItemById
};