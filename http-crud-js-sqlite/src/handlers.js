import { Sqlite, Variables } from "@fermyon/spin-sdk";
import { v4 as uuidv4 } from 'uuid';
import { validate as uuidValidate } from 'uuid';
const decoder = new TextDecoder();

const DB_NAME = "default";
const COMMAND_CREATE_ITEM = "INSERT INTO ITEMS (ID, NAME, ACTIVE) VALUES ($1, $2, $3)";
const COMMAND_READ_ALL_ITEMS = "SELECT ID, NAME, ACTIVE FROM ITEMS ORDER BY NAME";
const COMMAND_READ_SINGLE_ITEM = "SELECT ID, NAME, ACTIVE FROM ITEMS WHERE ID = $1";
const COMMAND_DELETE_SINGLE_ITEM = "DELETE FROM ITEMS WHERE ID = $1";
const COMMAND_DELETE_MANY_ITEMS = "DELETE FROM ITEMS WHERE ID IN";
const COMMAND_UPDATE_SINGLE_ITEM = "UPDATE ITEMS SET NAME = $1, ACTIVE = $2 WHERE ID = $3";

const DEFAULT_HEADERS = {
    "Content-Type": "application/json"
};

const getResponseHeaders = (defaultHeaders) => {
    let v = Variables.get("spin_runtime_name");
    let headers = {
        "X-Spin-Runtime": v
    }
    Object.assign(headers, defaultHeaders)
    return headers;
}

const getAllItems = (res) => {
    const db = Sqlite.open(DB_NAME);
    console.log("Loading all items from database");
    const queryResult = db.execute(COMMAND_READ_ALL_ITEMS, []);
    let items = queryResult.rows.map(row => {
        return {
            id: row["ID"],
            name: row["NAME"],
            active: row["ACTIVE"] == 1
        }
    });
    console.log(`Loaded ${items.length} items from database`);
    res.set(getResponseHeaders(DEFAULT_HEADERS));
    res.send(JSON.stringify(items));
};

const badRequest = (message, res) => {
    res.status(400);
    res.set(DEFAULT_HEADERS);
    res.send(JSON.stringify({
        message
    }));
};

const notFound = (message, res) => {
    res.status(404);
    res.set(DEFAULT_HEADERS);
    res.send(JSON.stringify({
        message
    }));
}

const getItemById = (id, res) => {
    if (!uuidValidate(id)) {
        return badRequest("Invalid identifier received via URL", res);
    }
    const db = Sqlite.open(DB_NAME);
    console.log(`Loading item with id ${id} from database`);
    let queryResult = db.execute(COMMAND_READ_SINGLE_ITEM, [id]);
    if (queryResult.rows.length == 0) {
        console.log(`Item with id ${id} not found in database`);
        return notFound(`No item found with id ${id}`, res);
    }
    let first = queryResult.rows[0];
    let found = {
        id: first["ID"],
        name: first["NAME"],
        active: first["ACTIVE"] == 1
    }
    console.log(`Found item with id ${id} in database`);
    res.set(getResponseHeaders(DEFAULT_HEADERS));
    res.send(JSON.stringify(found));
};

const deleteItemById = (id, res) => {
    if (!uuidValidate(id)) {
        return badRequest("Invalid identifier received via URL", res);
    }
    const db = Sqlite.open(DB_NAME);
    console.log(`Removing item with id ${id} from database`);
    db.execute(COMMAND_DELETE_SINGLE_ITEM, [id]);
    res.status(204);
    res.set(getResponseHeaders(DEFAULT_HEADERS));
    res.end();
};

const deleteManyItems = (requestBody, res) => {
    let payload = JSON.parse(decoder.decode(requestBody));
    if (!Array.isArray(payload.ids) ||
        payload.ids.length == 0 ||
        !payload.ids.every(id => uuidValidate(id))) {
        return badRequest("Invalid payload received. Expecting an array with valid uuids", res);
    }

    let cmd = `${COMMAND_DELETE_MANY_ITEMS} (`;
    let parameters = [];
    for (let i = 0; i < payload.ids.length; i++) {
        cmd = `${cmd}\$${i + 1}`;
        if (i < payload.ids.length - 1) {
            cmd = `${cmd},`;
        }
        parameters.push(payload.ids[i]);
    }
    cmd = `${cmd})`;
    const db = Sqlite.open(DB_NAME);
    console.log(`Removing items with ids (${payload.ids.join(',')}) from database`);
    db.execute(cmd, parameters);
    res.status(204);
    res.set(getResponseHeaders(DEFAULT_HEADERS))
    res.end();
};

const createItem = (baseUrl, requestBody, res) => {
    let payload = JSON.parse(decoder.decode(requestBody));

    if (!payload || !payload.name || typeof payload.active != "boolean") {
        return badRequest("Invalid payload received. Expecting {\"name\":\"some name\", \"active\": true}", res);
    }

    const newItem = {
        id: uuidv4(),
        name: payload.name,
        active: payload.active
    };
    const db = Sqlite.open(DB_NAME);
    console.log(`Inserting new item (${payload.name}) in database`);
    db.execute(COMMAND_CREATE_ITEM, [
        newItem.id,
        newItem.name,
        newItem.active ? 1 : 0
    ]);

    let customHeaders = {
        "Location": `${baseUrl}/items/${newItem.id}`
    };
    Object.assign(customHeaders, DEFAULT_HEADERS);

    res.status(201);
    res.set(getResponseHeaders(customHeaders))
    res.send(JSON.stringify(newItem));
};

const updateItemById = (baseUrl, id, requestBody, res) => {
    let payload = JSON.parse(decoder.decode(requestBody));
    if (!payload || !payload.name || typeof payload.active != "boolean") {
        return badRequest("Invalid payload received. Expecting {\"name\":\"some name\", \"active\": true}", res);
    }
    if (!uuidValidate(id)) {
        return badRequest("Invalid identifier received via URL", res);
    }
    const db = Sqlite.open(DB_NAME);
    console.log(`Updating item with id ${id} in database`);
    let item = {
        id: id,
        name: payload.name,
        active: payload.active,
    };
    db.execute(COMMAND_UPDATE_SINGLE_ITEM, [
        item.name,
        item.active ? 1 : 0,
        item.id
    ]);

    let customHeaders = {
        "Location": `${baseUrl}/items/${id}`
    };
    Object.assign(customHeaders, DEFAULT_HEADERS);

    res.set(getResponseHeaders(customHeaders));
    res.send(JSON.stringify(item));
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