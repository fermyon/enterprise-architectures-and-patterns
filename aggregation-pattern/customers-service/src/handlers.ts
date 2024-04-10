import { Sqlite } from "@fermyon/spin-sdk";
import { CustomerDetailsModel, CustomerListModel } from "./models";

const SQL_READ_ALL_CUSTOMERS = "SELECT Id, Name, Country, Scoring FROM Customers order by Name";
const SQL_READ_TOP_CUSTOMERS = "SELECT Id, Name, Country, Scoring FROM Customers order by Scoring desc, Name LIMIT ?";
const SQL_READ_CUSTOMER_BY_ID = "SELECT Id, Name, City, Country, Scoring FROM Customers WHERE Id=?";
const SQL_READ_CUSTOMER_COUNT = "SELECT COUNT(Id) AS CustomerCount FROM Customers";
const DEFAULT_HEADERS = {
    "Content-Type": "application/json"
};

export function getAllItems() {
    const con = Sqlite.openDefault();
    const result = con.execute(SQL_READ_ALL_CUSTOMERS, []);
    const items = result.rows.map((row) => {
        return {
            id: row["Id"]?.toString(),
            name: row["Name"]?.toString(),
            country: row["Country"]?.toString(),
            scoring: Number(row["Scoring"]?.toString())

        } as CustomerListModel
    });
    return {
        status: 200,
        headers: DEFAULT_HEADERS,
        body: JSON.stringify(items)
    };
}

export function getTopCustomers(limit: string) {
    if (Number.isNaN(+limit)) {
        return badRequest("Invalid parameter (limit) received")
    }
    let l = +limit;
    if (l < 1) {
        return badRequest("Limit must be higher than 0")
    }
    const con = Sqlite.openDefault();
    const result = con.execute(SQL_READ_TOP_CUSTOMERS, [l]);
    const items = result.rows.map((row) => {
        return {
            id: row["Id"]?.toString(),
            name: row["Name"]?.toString(),
            country: row["Country"]?.toString(),
            scoring: Number(row["Scoring"]?.toString())

        } as CustomerListModel
    });
    return {
        status: 200,
        headers: DEFAULT_HEADERS,
        body: JSON.stringify(items)
    };
}

export function getCustomerCount() {
    const con = Sqlite.openDefault();
    const result = con.execute(SQL_READ_CUSTOMER_COUNT, []);
    const count = result.rows[0]["CustomerCount"];
    return {
        status: 200,
        headers: DEFAULT_HEADERS,
        body: JSON.stringify({
            count: Number(count)
        })
    };
}

export function getCustomerById(id: string) {
    if (!id || !isGuid(id)) {
        return badRequest("Invalid parameter (id) received")
    }



    const con = Sqlite.openDefault();
    const result = con.execute(SQL_READ_CUSTOMER_BY_ID, [id]);
    if (result.rows.length === 0) {
        return {
            status: 404
        };
    }
    const found = {
        id: result.rows[0]["Id"]?.toString(),
        name: result.rows[0]["Name"]?.toString(),
        city: result.rows[0]["City"]?.toString(),
        country: result.rows[0]["Country"]?.toString(),
        scoring: Number(result.rows[0]["Scoring"]?.toString())
    } as CustomerDetailsModel;

    return {
        status: 200,
        headers: DEFAULT_HEADERS,
        body: JSON.stringify(found)
    };
}

function badRequest(message: string) {
    return {
        status: 400,
        headers: DEFAULT_HEADERS,
        body: JSON.stringify({
            message
        })
    };
};


function isGuid(id: string) {
    let m = id.match('^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$');
    console.log(JSON.stringify(m));
    return !!m;
}